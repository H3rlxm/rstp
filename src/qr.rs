pub struct OtpAuth {
    pub label: String,
    pub issuer: String,
    pub secret: String,
}

pub fn parse_qr(path: &str) -> Option<OtpAuth> {
    let img = match image::open(path) {
        Ok(i) => i.to_luma8(),
        Err(_) => return None,
    };

    let mut decoder = rqrr::PreparedImage::prepare(img);
    let grids = decoder.detect_grids();
    if grids.is_empty() {
        return None;
    }

    let data = grids[0].decode().ok()?;
    parse_otpauth_uri(&data.1)
}

pub fn parse_otpauth_uri(uri: &str) -> Option<OtpAuth> {
    let uri = uri.trim();
    if !uri.starts_with("otpauth://") {
        return parse_raw_secret(uri);
    }

    let without_scheme = uri.strip_prefix("otpauth://")?;
    let (typ, rest) = without_scheme.split_once('/')?;
    if typ != "totp" && typ != "hotp" {
        return None;
    }

    let (label, query) = rest.split_once('?')?;
    let label = label.replace('+', " ").replace("%20", " ");
    let label = url_decode(&label);

    let params: std::collections::HashMap<String, String> = query
        .split('&')
        .filter_map(|p| p.split_once('='))
        .map(|(k, v)| (k.to_lowercase(), url_decode(v)))
        .collect();

    let secret = params.get("secret")?.to_uppercase();
    let issuer = params.get("issuer").cloned().unwrap_or_default();

    Some(OtpAuth { label, issuer, secret })
}

fn parse_raw_secret(s: &str) -> Option<OtpAuth> {
    let s = s.trim().replace(' ', "");
    let clean = s.to_uppercase();
    if clean.chars().all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567=".contains(c)) && clean.len() >= 16 {
        Some(OtpAuth {
            label: "Manual".to_string(),
            issuer: "".to_string(),
            secret: clean,
        })
    } else {
        None
    }
}

fn url_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push_str(&hex);
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn is_image_file(path: &str) -> bool {
    let lower = path.to_lowercase();
    lower.ends_with(".png") || lower.ends_with(".jpg") || lower.ends_with(".jpeg") || lower.ends_with(".bmp") || lower.ends_with(".webp")
}
