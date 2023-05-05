mod users;

use axum::{Router, Extension};

use crate::{control::sessions::post_sessions, service::{sessions::HashSessionService, hash::BcryptHashService, users::HashUserService}, repository::{sessions::HttpSessionRepository, users::HttpUserRepository}, constants::{RESOURCE_MANAGEMENT_URL, SESSION_ID_GEN_RETRIES}};

use self::users::users_router;

pub fn main_router() -> Router {
    let users_url = RESOURCE_MANAGEMENT_URL.clone() + "/users";
    let sessions_url = RESOURCE_MANAGEMENT_URL.clone() + "/sessions";
    
    let users_service = HashUserService::new(
        HttpUserRepository::new(users_url.as_str()),
        BcryptHashService::new()
    );
    let sessions_service = HashSessionService::new(
        HttpSessionRepository::new(sessions_url.as_str()),
        HttpUserRepository::new(users_url.as_str()),
        BcryptHashService::new(),
        *SESSION_ID_GEN_RETRIES
    );

    let sessions_handler = axum::routing::post(post_sessions::<HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>>);

    Router::new()
        .nest("/users", users_router(users_service))
        .route("/sessions", sessions_handler)
        .layer(Extension(sessions_service))
}
