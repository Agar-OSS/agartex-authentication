use std::fmt::Debug;

use axum::{Extension, http::StatusCode};
use tracing::info;

use crate::{service::{users::{UserService, UserCreationError}}, validation::ValidatedJson, domain::users::Credentials};

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
