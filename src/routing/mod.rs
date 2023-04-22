use axum::{Router, Extension};
use sqlx::PgPool;

use crate::{control::{sessions::post_sessions, users::get_users}, service::{sessions::HashSessionService, hash::BcryptHashService}, repository::{sessions::PgSessionRepository, users::PgUserRepository}};

pub fn main_router(pool: &PgPool) -> Router {
    let session_service = HashSessionService::new(
        PgSessionRepository::new(pool),
        PgUserRepository::new(pool),
        BcryptHashService::new()
    );

    let users_handler = axum::routing::get(get_users::<HashSessionService<PgSessionRepository, PgUserRepository, BcryptHashService>>);
    let sessions_handler = axum::routing::post(post_sessions::<HashSessionService<PgSessionRepository, PgUserRepository, BcryptHashService>>);

    Router::new()
        .route("/users", users_handler)
        .route("/sessions", sessions_handler)
        .layer(Extension(session_service))
}
