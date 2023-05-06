use std::str::FromStr;

use axum::async_trait;
use http::StatusCode;
use mockall::automock;
use reqwest::{Client, Url};
use tracing::{error, warn};

use crate::domain::sessions::{Session, SessionData};

pub enum SessionGetError {
    Missing,
    Unknown
}

pub enum SessionDeleteError {
    Unknown
}

pub enum SessionInsertError {
    Duplicate,
    Unknown
}

#[automock]
#[async_trait]
pub trait SessionRepository {
    async fn insert(&self, session_data: &SessionData) -> Result<(), SessionInsertError>;
    async fn get(&self, id: &str) -> Result<Session, SessionGetError>;
    async fn delete(&self, id: &str) -> Result<(), SessionDeleteError>;
}

#[derive(Debug, Clone)]
pub struct HttpSessionRepository {
    manager_sessions_url: Url,
    client: Client
}

impl HttpSessionRepository {
    pub fn new(url: &str) -> Self {
        Self { 
            manager_sessions_url: Url::from_str(url).unwrap(),
            client: Client::new()
        }
    }
}

#[async_trait]
impl SessionRepository for HttpSessionRepository {
    #[tracing::instrument(skip_all, fields(user_id = session_data.user_id))]
    async fn insert(&self, session_data: &SessionData) -> Result<(), SessionInsertError> {
        let req = self.client
            .post(self.manager_sessions_url.clone())
            .json(&session_data);


        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(SessionInsertError::Unknown);
            }
        };

        match res.status() {
            StatusCode::CREATED => Ok(()),
            StatusCode::CONFLICT => {
                warn!("Duplicate session {:?}", session_data);
                Err(SessionInsertError::Duplicate)
            },
            code => {
                error!("Unexpected code {:?}", code);
                Err(SessionInsertError::Unknown)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn get(&self, id: &str) -> Result<Session, SessionGetError> {
        let req = self.client
            .get(self.manager_sessions_url.clone())
            .bearer_auth(id);
    
        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(SessionGetError::Unknown);
            }
        };

        let body = match res.status() {
            StatusCode::OK => res.json::<Session>(),
            StatusCode::NOT_FOUND => {
                warn!("Missing session {:?}", id);
                return Err(SessionGetError::Missing);
            },
            code => {
                error!("Unexpected code {:?}", code);
                return Err(SessionGetError::Unknown);
            }
        };

        body.await.map_err(|err| {
            error!(%err);
            SessionGetError::Unknown
        })
    }

    #[tracing::instrument(skip_all)]
    async fn delete(&self, id: &str) -> Result<(), SessionDeleteError> {
        let req = self.client
            .delete(self.manager_sessions_url.clone())
            .bearer_auth(id);
    
        let res = match req.send().await {
            Ok(res) => res,
            Err(err) => {
                error!(%err);
                return Err(SessionDeleteError::Unknown);
            }
        };

        match res.status() {
            StatusCode::OK => Ok(()),
            code => {
                error!("Unexpected code {:?}", code);
                Err(SessionDeleteError::Unknown)
            }
        }
    }
}
