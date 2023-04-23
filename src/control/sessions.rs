use std::fmt::Debug;

use axum::{Extension, Json, http::StatusCode};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use cookie::time::OffsetDateTime;
use tracing::info;

use crate::{domain::users::Credentials, service::sessions::{SessionService, LoginError}, constants::SESSION_COOKIE_NAME};

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
