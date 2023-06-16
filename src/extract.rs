use axum::headers::{Header, Error};
use http::{HeaderName, HeaderValue};

use crate::constants::USER_HEADER_NAME;
pub struct XUserId(pub i32);

impl Header for XUserId {
    fn name() -> &'static HeaderName {
        &USER_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        if let Ok(s) = values.next().ok_or_else(Error::invalid)?.to_str() {
            if let Ok(num) = s.parse::<i32>() {
                return Ok(XUserId(num));
            }
        }
        Err(Error::invalid())
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let value = HeaderValue::from(self.0);

        values.extend(std::iter::once(value));
    }
}
