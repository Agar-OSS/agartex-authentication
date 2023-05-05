use axum::{Router, Extension};

use crate::{control::users::{post_users, get_users}, service::{users::HashUserService, hash::BcryptHashService, sessions::HashSessionService}, repository::{users::HttpUserRepository, sessions::HttpSessionRepository}};

pub fn users_router(
    users_service: HashUserService<HttpUserRepository, BcryptHashService>
) -> Router {
    let users_handler = axum::routing::get(get_users::<HashSessionService<HttpSessionRepository, HttpUserRepository, BcryptHashService>>)
        .post(post_users::<HashUserService<HttpUserRepository, BcryptHashService>>);
    
    Router::new()
        .route("/", users_handler)
        .layer(Extension(users_service))
}
