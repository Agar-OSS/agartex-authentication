use axum::async_trait;
use http::StatusCode;
use mockall::automock;
use reqwest::Client;
use tracing::{error, info};

use crate::domain::users::{User, Credentials};

pub enum UserGetError {
    Missing,
    Unknown
}

pub enum UserInsertError {
    Duplicate,
    Unknown
}

#[automock]
#[async_trait]
pub trait UserRepository {
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError>;
    async fn insert(&self, credentials: Credentials) -> Result<(), UserInsertError>;
}


#[derive(Debug, Clone)]
pub struct HttpUserRepository {
    manager_users_url: String,
    client: Client
}

impl HttpUserRepository {
    pub fn new(url: &str) -> Self {
        Self {
            manager_users_url: String::from(url),
            client: Client::new()
        }
    }
}

#[async_trait]
impl UserRepository for HttpUserRepository {
    #[tracing::instrument(skip(self))]
    async fn insert(&self, credentials: Credentials) -> Result<(), UserInsertError> {
        let req = self.client
            .post(self.manager_users_url.as_str())
            .json(&credentials);

        info!(?req);
        
        match req.send().await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(%err);
                let status = match err.status() {
                    None => return Err(UserInsertError::Unknown),
                    Some(status) => status
                };

                match status {
                    StatusCode::CONFLICT => Err(UserInsertError::Duplicate),
                    _ => Err(UserInsertError::Unknown)
                }
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError> {
        let url = self.manager_users_url.clone() + "/" + email;
        
        let req = self.client
            .get(url);

        info!(?req);

        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                let status = match err.status() {
                    None => return Err(UserGetError::Unknown),
                    Some(status) => status
                };

                return match status {
                    StatusCode::NOT_FOUND => Err(UserGetError::Missing),
                    _ => Err(UserGetError::Unknown)
                }
            }
        };

        res.json::<User>().await.map_err(|err| {
            error!(%err);
            UserGetError::Unknown
        })
    }
}
