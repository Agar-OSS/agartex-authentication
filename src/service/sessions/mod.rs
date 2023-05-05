use axum::async_trait;
use mockall::automock;
use rand::{Rng, distributions::Alphanumeric};
use chrono::{Utc, NaiveDateTime, DateTime};
use tracing::{warn, error, info};

use crate::{domain::{users::{Credentials, User}, sessions::SessionData}, repository::{sessions::{SessionRepository, SessionInsertError, SessionGetError}, users::{UserRepository, UserGetError}}, constants::{SESSION_LENGTH_SECONDS, SESSION_ID_LENGTH}};

use super::hash::HashService;

#[derive(PartialEq, Debug)]
pub enum LoginError {
    NoUser,
    Unknown
}

#[derive(PartialEq, Debug)]
pub enum SessionVerifyError {
    Missing,
    Unknown
}

#[automock]
#[async_trait]
pub trait SessionService {
    async fn login(&self, credentials: Credentials) -> Result<SessionData, LoginError>;
    async fn verify(&self, id: &str) -> Result<User, SessionVerifyError>;
}

#[derive(Debug, Clone)]
pub struct HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    session_repository: S,
    user_repository: U,
    hash_service: H,
    max_retries: u32
}

impl<S, U, H> HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    pub fn new(session_repository: S, user_repository: U, hash_service: H, max_retries: u32) -> Self {
        Self { session_repository, user_repository, hash_service, max_retries }
    }

    pub fn generate_session_id(id_len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(id_len)
            .map(char::from)
            .collect()
    }
}

#[async_trait]
impl<S, U, H> SessionService for HashSessionService<S, U, H>
where
    S: SessionRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    H: HashService + Send + Sync
{
    #[tracing::instrument(skip_all, field(email = credentials.email))]
    async fn login(&self, credentials: Credentials) -> Result<SessionData, LoginError> {
        info!("Attempting to login user");
        let user = match self.user_repository.get_by_email(&credentials.email).await {
            Ok(user) => user,
            Err(UserGetError::Missing) => {
                warn!("Login attempt failed");
                return Err(LoginError::NoUser)
            },
            Err(UserGetError::Unknown) => return Err(LoginError::Unknown)
        };

        match self.hash_service.verify(&credentials.password, &user.password_hash) {
            Err(err) => {
                error!(%err);
                return Err(LoginError::Unknown);
            },
            Ok(false) => {
                warn!("Login attempt failed");
                return Err(LoginError::NoUser);
            },
            Ok(true) => ()
        };


        for _ in 0..self.max_retries {
            let session_data = SessionData {
                id: Self::generate_session_id(SESSION_ID_LENGTH),
                user_id: user.id,
                expires: Utc::now().timestamp() + *SESSION_LENGTH_SECONDS
            };
            
            match self.session_repository.insert(&session_data).await {
                Ok(()) => {
                    info!("Login attempt succeeded");
                    return Ok(session_data);
                }
                Err(SessionInsertError::Duplicate) => continue,
                Err(SessionInsertError::Unknown) => return Err(LoginError::Unknown)
            }
        };

        Err(LoginError::Unknown)
    }

    async fn verify(&self, id: &str) -> Result<User, SessionVerifyError> {
        let session = match self.session_repository.get(id).await {
            Ok(session) => session,
            Err(SessionGetError::Missing) => return Err(SessionVerifyError::Missing),
            Err(SessionGetError::Unknown) => return Err(SessionVerifyError::Unknown)
        };

        let expires = match NaiveDateTime::from_timestamp_opt(session.expires, 0) {
            Some(expires) => expires,
            None => {
                return Err(match self.session_repository.delete(id).await {
                    Ok(()) => SessionVerifyError::Missing,
                    Err(_) => SessionVerifyError::Unknown
                });
            }
        };

        if DateTime::<Utc>::from_utc(expires, Utc) < Utc::now() {
            return Err(match self.session_repository.delete(id).await {
                Ok(()) => SessionVerifyError::Missing,
                Err(_) => SessionVerifyError::Unknown
            });
        }

        Ok(session.user)
    }
}

#[cfg(test)]
mod tests;
