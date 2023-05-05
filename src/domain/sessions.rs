use serde::{Serialize, Deserialize};

use super::users::User;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Session {
    pub id: String,
    pub user: User,
    pub expires: i64
}


#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SessionData {
    pub id: String,
    pub user_id: i32,
    pub expires: i64
}
