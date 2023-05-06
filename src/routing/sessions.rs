use axum::{Router, routing, Extension};

use crate::{service::{sessions::HashSessionService, hash::BcryptHashService}, repository::{sessions::HttpSessionRepository, users::HttpUserRepository}, control::sessions::{get_sessions, post_sessions, delete_sessions}};

pub fn sessions_router(sessions_service: HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>) -> Router {
    let root_handler = routing
        ::get(get_sessions::<HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>>)
        .post(post_sessions::<HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>>)
        .delete(delete_sessions::<HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>>);

    Router::new()
        .route("/", root_handler)
        .layer(Extension(sessions_service))
}
