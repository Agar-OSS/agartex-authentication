use std::fmt::Debug;

use axum::{Extension, http::StatusCode, response::{AppendHeaders, IntoResponse}};
use axum_extra::extract::{CookieJar};
use tracing::{info, warn};

use crate::{service::{sessions::{SessionService, SessionVerifyError}, users::{UserService, UserCreationError}}, constants::{SESSION_COOKIE_NAME, USER_ID_HEADER}, validation::ValidatedJson, domain::users::Credentials};

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

#[tracing::instrument(skip_all, fields(email = credentials.email))]
pub async fn post_users<T: UserService + Debug>(
    Extension(service): Extension<T>,
    ValidatedJson(credentials): ValidatedJson<Credentials>
) -> StatusCode {
    info!("Received registration attempt");

    match service.register(credentials).await {
        Ok(()) => StatusCode::CREATED,
        Err(UserCreationError::DuplicateEmail) => StatusCode::CONFLICT,
        Err(UserCreationError::Unknown) => StatusCode::INTERNAL_SERVER_ERROR
    }
}
