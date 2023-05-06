use serde::{Serialize, Deserialize};
use validator::{Validate, validate_length, ValidationErrors, ValidationError};

use crate::constants::SESSION_ID_LENGTH;

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

pub struct SessionId(pub String);

impl Validate for SessionId {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errs = ValidationErrors::new();
        if !validate_length(&self.0, None, None, Some(SESSION_ID_LENGTH as u64)) {
            errs.add("id", ValidationError::new("length"));
        }
        if !self.0.chars().all(|c| c.is_ascii_alphanumeric()) {
            errs.add("id", ValidationError::new("charset"));
        }

        if errs.is_empty() {
            Ok(())
        } else {
            Err(errs)
        }
    }
}
