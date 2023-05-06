use std::fmt::Debug;

use axum::{Extension, Json, http::StatusCode, response::AppendHeaders};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use cookie::time::OffsetDateTime;
use tracing::{error, info, warn};
use validator::Validate;

use crate::{domain::{users::Credentials, sessions::SessionId}, service::sessions::{SessionService, LoginError, SessionVerifyError, LogoutError}, constants::{SESSION_COOKIE_NAME, USER_ID_HEADER}};

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar,
    Json(credentials): Json<Credentials>
) -> Result<(CookieJar, StatusCode), StatusCode> {
    info!("Received login attempt");
    let session = match service.login(credentials).await {
        Err(LoginError::NoUser) =>  {
            warn!("Bad credentials provided");
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(LoginError::Unknown) => {
            error!("Unexpected error during login attempt");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
        Ok(session) => session
    };

    let cookie = Cookie::build(SESSION_COOKIE_NAME.as_str(), session.id)
        .expires(OffsetDateTime::from_unix_timestamp(session.expires).unwrap())
        .http_only(true)
        // .secure(true) <-- add this when TLS is set up
        .finish();

    Ok((jar.add(cookie), StatusCode::CREATED))
}

#[tracing::instrument(skip_all)]
pub async fn get_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar
) -> Result<AppendHeaders<[(&'static str, i32); 1]>, StatusCode> {
    info!("Received session verification attempt");
    let session_id = match jar.get(SESSION_COOKIE_NAME.as_str()) {
        Some(cookie) => cookie.value(),
        None => {
            warn!("No session provided!");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if let Err(errs) = SessionId(String::from(session_id)).validate() {
        warn!("Session ID: {}, Validation errors: {}", session_id, errs);
        return Err(StatusCode::UNPROCESSABLE_ENTITY)
    }

    let user = match service.verify(session_id).await {
        Err(SessionVerifyError::Missing) =>  {
            warn!("Provided session is not valid: {}", session_id);
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(SessionVerifyError::Unknown) => {
            error!("Unexpected error during session verification attempt");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        },
        Ok(session) => session
    };

    let headers = AppendHeaders([
        (USER_ID_HEADER.as_str(), user.id)
    ]);

    info!("Successfully verified session");
    Ok(headers)
}

#[tracing::instrument(skip_all)]
pub async fn delete_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar
) -> StatusCode {
    info!("Received logout attempt");
    let session_id = match jar.get(SESSION_COOKIE_NAME.as_str()) {
        Some(cookie) => cookie.value(),
        None => {
            warn!("No session provided!");
            return StatusCode::UNAUTHORIZED;
        }
    };

    if let Err(errs) = SessionId(String::from(session_id)).validate() {
        warn!("Session ID: {}, Validation errors: {}", session_id, errs);
        return StatusCode::UNPROCESSABLE_ENTITY;
    }

    match service.logout(session_id).await {
        Ok(()) => {
            info!("Successfully logged out session {}", session_id);
            StatusCode::OK
        },
        Err(LogoutError::Unknown) => {
            error!("Unable to process logout attempt");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests;
