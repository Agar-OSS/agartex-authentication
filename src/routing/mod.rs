mod users;

use axum::{Router, Extension};
use sqlx::PgPool;

use crate::{control::sessions::post_sessions, service::{sessions::HashSessionService, hash::BcryptHashService}, repository::{sessions::PgSessionRepository, users::HttpUserRepository}, constants::RESOURCE_MANAGEMENT_URL};

use self::users::users_router;

pub fn main_router(pool: &PgPool) -> Router {
    let users_url = RESOURCE_MANAGEMENT_URL.clone() + "/users";
    
    let sessions_service = HashSessionService::new(
        PgSessionRepository::new(pool),
        HttpUserRepository::new(users_url.as_str()),
        BcryptHashService::new()
    );

    let sessions_handler = axum::routing::post(post_sessions::<HashSessionService<PgSessionRepository, HttpUserRepository, BcryptHashService>>);

    Router::new()
        .nest("/users", users_router(users_url.as_str()))
        .route("/sessions", sessions_handler)
        .layer(Extension(sessions_service))
}
