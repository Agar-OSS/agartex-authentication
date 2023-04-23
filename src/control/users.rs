use std::fmt::Debug;

use axum::{Extension, http::StatusCode, response::{AppendHeaders, IntoResponse}};
use axum_extra::extract::{CookieJar};
use tracing::{info, warn};

use crate::{service::sessions::{SessionService, SessionVerifyError}, constants::{SESSION_COOKIE_NAME, USER_ID_HEADER}};

#[tracing::instrument(skip(service))]
pub async fn get_users<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar
) -> Result<impl IntoResponse, StatusCode> {
    info!("Received login attempt");
    let session_id = match jar.get(SESSION_COOKIE_NAME.as_str()) {
        Some(cookie) => cookie.value(),
        None => {
            warn!("No session ID provided!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let user = match service.verify(session_id).await {
        Err(SessionVerifyError::Missing) =>  {
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(SessionVerifyError::Unknown) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
        Ok(session) => session
    };

    let headers = AppendHeaders([
        (USER_ID_HEADER.as_str(), user.id)
    ]);

    Ok(headers)
}
