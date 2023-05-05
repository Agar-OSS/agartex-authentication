mod sessions;
mod users;

use axum::Router;

use crate::{service::{sessions::HashSessionService, hash::BcryptHashService, users::HashUserService}, repository::{sessions::HttpSessionRepository, users::HttpUserRepository}, constants::{RESOURCE_MANAGEMENT_URL, SESSION_ID_GEN_RETRIES}};

use self::{users::users_router, sessions::sessions_router};

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

    Router::new()
        .nest("/users", users_router(users_service))
        .nest("/sessions", sessions_router(sessions_service))
}
