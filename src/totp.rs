use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha1 = Hmac<Sha1>;

pub fn generate_totp(secret_b32: &str, period: u64, digits: u32) -> String {
    let secret = match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret_b32) {
        Some(s) => s,
        None => return "INVALID KEY".to_string(),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let counter = now / period;

    let mut mac = HmacSha1::new_from_slice(&secret).expect("HMAC key");
    mac.update(&counter.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let offset = (result[19] & 0x0f) as usize;
    let code = ((result[offset] as u32 & 0x7f) << 24)
        | ((result[offset + 1] as u32) << 16)
        | ((result[offset + 2] as u32) << 8)
        | (result[offset + 3] as u32);

    format!("{:0width$}", code % 10u32.pow(digits), width = digits as usize)
}

pub fn remaining_seconds(period: u64) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    period - (now % period)
}
