use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Credentials {
    pub email: String,
    pub password: String
}
