use rusqlite::{Connection, params};

pub struct Account {
    pub id: i64,
    pub label: String,
    pub issuer: String,
    pub secret: String,
    pub algorithm: String,
    pub digits: u32,
    pub period: u64,
}

pub fn db_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let dir = std::path::PathBuf::from(home).join(".config/rsotp");
    std::fs::create_dir_all(&dir).ok();
    dir.join("rsotp-codes.db")
}

pub fn init_db() -> Connection {
    let path = db_path();
    let conn = Connection::open(&path).expect("Failed to open database");

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            label TEXT NOT NULL,
            issuer TEXT NOT NULL DEFAULT '',
            secret TEXT NOT NULL,
            algorithm TEXT NOT NULL DEFAULT 'SHA1',
            digits INTEGER NOT NULL DEFAULT 6,
            period INTEGER NOT NULL DEFAULT 30,
            created_at TEXT DEFAULT (datetime('now'))
        );"
    ).expect("Failed to create table");

    conn
}

pub fn list_accounts(conn: &Connection) -> Vec<Account> {
    let mut stmt = conn.prepare(
        "SELECT id, label, issuer, secret, algorithm, digits, period FROM accounts ORDER BY label"
    ).expect("Failed to prepare query");

    stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            label: row.get(1)?,
            issuer: row.get(2)?,
            secret: row.get(3)?,
            algorithm: row.get(4)?,
            digits: row.get::<_, i32>(5)? as u32,
            period: row.get::<_, i64>(6)? as u64,
        })
    }).expect("Failed to query accounts")
    .filter_map(|r| r.ok())
    .collect()
}

pub fn add_account(conn: &Connection, label: &str, issuer: &str, secret: &str) {
    conn.execute(
        "INSERT INTO accounts (label, issuer, secret) VALUES (?1, ?2, ?3)",
        params![label, issuer, secret],
    ).expect("Failed to insert account");
}

pub fn delete_account(conn: &Connection, id: i64) {
    conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])
        .expect("Failed to delete account");
}
