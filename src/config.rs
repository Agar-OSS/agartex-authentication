use std::{io::BufReader, fs::File, path::Path, net::SocketAddr};

use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub database_addr: String,
    pub session_cookie_name: String,
    pub session_length_seconds: i64,
    pub user_id_header: String
}

const CONFIG_PATH: &str = "config.json";

pub const SERVER_ADDR_ENV_VAR: &str = "SERVER_ADDR";
pub const DATABASE_ADDR_ENV_VAR: &str = "DATABASE_ADDR";
pub const SESSION_COOKIE_NAME_VAR: &str = "SESSION_COOKIE_NAME";
pub const SESSION_LENGTH_SECONDS_VAR: &str = "SESSION_LENGTH_SECONDS";
pub const USER_ID_HEADER_VAR: &str = "USER_ID_HEADER";

fn load_config() -> Config {
    let mut cfg: Config = serde_json::from_reader(BufReader::new(File::open(Path::new(CONFIG_PATH)).unwrap())).unwrap();

    if let Ok(server_addr) = std::env::var(SERVER_ADDR_ENV_VAR) {
        cfg.server_addr = server_addr.parse().unwrap();
    }

    if let Ok(database_addr) = std::env::var(DATABASE_ADDR_ENV_VAR) {
        cfg.database_addr = database_addr.parse().unwrap();
    }

    if let Ok(session_cookie_name) = std::env::var(SESSION_COOKIE_NAME_VAR) {
        cfg.session_cookie_name = session_cookie_name.parse().unwrap();
    }

    if let Ok(session_length_seconds) = std::env::var(SESSION_LENGTH_SECONDS_VAR) {
        cfg.session_length_seconds = session_length_seconds.parse().unwrap();
    }

    if let Ok(user_id_header) = std::env::var(USER_ID_HEADER_VAR) {
        cfg.user_id_header = user_id_header.parse().unwrap();
    }

    cfg
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
}

