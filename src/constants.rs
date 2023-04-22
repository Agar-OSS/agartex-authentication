use lazy_static::lazy_static;
use regex::Regex;

pub const HASH_COST: u32 = 12;
pub const PASSWORD_SPECIAL_CHARS: &str = "!@#$%^&*";

lazy_static! {
    pub static ref PASSWORD_REGEX: Regex = Regex::new(format!("^[A-Za-z0-9{}]*$", PASSWORD_SPECIAL_CHARS).as_str()).unwrap();
}
