use axum::{Router, Extension};

use crate::{control::users::{post_users, get_users}, service::{users::HashUserService, hash::BcryptHashService, sessions::HashSessionService}, repository::{users::HttpUserRepository, sessions::PgSessionRepository}};

pub fn users_router(users_url: &str) -> Router {
    
    let users_service = HashUserService::new(
        HttpUserRepository::new(users_url),
        BcryptHashService::new()
    );

    
    let users_handler = axum::routing::get(get_users::<HashSessionService<PgSessionRepository, HttpUserRepository, BcryptHashService>>)
        .post(post_users::<HashUserService<HttpUserRepository, BcryptHashService>>);
    
    Router::new()
        .route("/", users_handler)
        .layer(Extension(users_service))
}