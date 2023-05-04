use std::{env, str::FromStr, fmt::Debug, net::{SocketAddr, Ipv4Addr, IpAddr}};

use lazy_static::lazy_static;
use regex::Regex;

fn load_env_or_default<T>(var: &str, default: T) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    match env::var(var) {
        Ok(val) => T::from_str(&val).unwrap(),
        Err(_) => default
    }
}

// implicit environment variables used:
// - PGHOST
// - PGPORT
// - PGDATABASE
// - PGUSER
// - PGPASSWORD
pub const FALLBACK_DB_URL: &str = "postgres://localhost:5432/agartex-db";
pub const PASSWORD_SPECIAL_CHARS: &str = "!@#$%^&*";

lazy_static! {
    pub static ref HASH_COST: u32 = load_env_or_default("BCRYPT_HASH_COST", 12);
    pub static ref SERVER_URL: SocketAddr = load_env_or_default("SERVER_URL", SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3100));
    pub static ref SESSION_COOKIE_NAME: String = load_env_or_default("SESSION_COOKIE_NAME", String::from("RSESSID"));
    pub static ref SESSION_LENGTH_SECONDS: i64 = load_env_or_default("SESSION_LENGTH_SECONDS", 60 * 60 * 24 * 30); // 30 days
    pub static ref USER_ID_HEADER: String = load_env_or_default("USER_ID_HEADER", String::from("X-User-Id"));
    pub static ref RESOURCE_MANAGEMENT_URL: String = load_env_or_default("RESOURCE_MANAGEMENT_URL", String::from("http://localhost:3200"));
    
    pub static ref PASSWORD_REGEX: Regex = Regex::new(format!("^[A-Za-z0-9{}]*$", PASSWORD_SPECIAL_CHARS).as_str()).unwrap();
}
