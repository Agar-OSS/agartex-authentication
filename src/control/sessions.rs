use std::fmt::Debug;

use axum::{Extension, Json, http::StatusCode, response::{IntoResponse, AppendHeaders}};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use cookie::time::OffsetDateTime;
use tracing::{info, warn};

use crate::{domain::users::Credentials, service::sessions::{SessionService, LoginError, SessionVerifyError}, constants::{SESSION_COOKIE_NAME, USER_ID_HEADER}};

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_sessions<T: SessionService + Debug>(
    Extension(service): Extension<T>,
    jar: CookieJar,
    Json(credentials): Json<Credentials>
) -> Result<(CookieJar, StatusCode), StatusCode> {
    info!("Received login attempt");
    let session = match service.login(credentials).await {
        Err(LoginError::NoUser) =>  {
            return Err(StatusCode::UNAUTHORIZED);
        },
        Err(LoginError::Unknown) => {
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