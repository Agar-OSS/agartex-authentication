use std::str::FromStr;

use axum::async_trait;
use http::StatusCode;
use mockall::automock;
use reqwest::{Client, Url};
use tracing::{error, warn};

use crate::domain::users::{User, UserData};

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
    async fn insert(&self, user_data: UserData) -> Result<(), UserInsertError>;
}


#[derive(Debug, Clone)]
pub struct HttpUserRepository {
    manager_users_url: Url,
    client: Client
}

impl HttpUserRepository {
    pub fn new(url: &str) -> Self {
        Self {
            manager_users_url: Url::from_str(url).unwrap(),
            client: Client::new()
        }
    }
}

#[async_trait]
impl UserRepository for HttpUserRepository {
    #[tracing::instrument(skip_all, fields(email = user_data.email))]
    async fn insert(&self, user_data: UserData) -> Result<(), UserInsertError> {
        let req = self.client
            .post(self.manager_users_url.clone())
            .json(&user_data);

        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(UserInsertError::Unknown);
            }
        };

        match res.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => {
                warn!("Duplicate user {:?}", user_data);
                Err(UserInsertError::Duplicate)
            },
            code => {
                error!("Unexpected code {:?}", code);
                Err(UserInsertError::Unknown)
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_email(&self, email: &str) -> Result<User, UserGetError> {
        let mut url = self.manager_users_url.clone();
        match url.path_segments_mut() {
            Ok(mut path) => path.extend([email]),
            Err(_) => {
                error!("Bad Resource Management URL: {:?}", self.manager_users_url);
                return Err(UserGetError::Unknown);
            }
        };
        
        let req = self.client.get(url);

        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(UserGetError::Unknown);
            }
        };

        let body = match res.status() {
            StatusCode::OK => res.json::<User>(),
            StatusCode::NOT_FOUND => {
                warn!("Missing user {:?}", email);
                return Err(UserGetError::Missing);
            },
            code => return {
                error!(%code);
                Err(UserGetError::Unknown)
            }
        };

        body.await.map_err(|err| {
            error!(%err);
            UserGetError::Unknown
        })
    }
}
