use axum::{Router, Extension};

use crate::{control::users::post_users, service::{users::HashUserService, hash::BcryptHashService}, repository::users::HttpUserRepository};

pub fn users_router(
    users_service: HashUserService<HttpUserRepository, BcryptHashService>
) -> Router {
    let users_handler = axum::routing::post(post_users::<HashUserService<HttpUserRepository, BcryptHashService>>);
    
    Router::new()
        .route("/", users_handler)
        .layer(Extension(users_service))
}
