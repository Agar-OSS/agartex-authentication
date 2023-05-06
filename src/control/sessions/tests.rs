use chrono::Utc;
use mockall::predicate;

use crate::{service::sessions::MockSessionService, domain::{sessions::SessionData, users::User}, constants::SESSION_ID_LENGTH};

use super::*;

fn mock_email() -> String {
    String::from("email")
}

fn mock_password() -> String {
    String::from("password")
}

fn mock_credentials() -> Credentials {
    Credentials {
        email: mock_email(),
        password: mock_password()
    }
}

fn mock_session_id() -> String {
    String::from_iter(std::iter::repeat('1').take(SESSION_ID_LENGTH))
}

fn mock_user() -> User {
    User {
        id: 1, 
        email: mock_email(),
        password_hash: mock_password()
    }
}

fn mock_session_data() -> SessionData {
    SessionData {
        id: mock_session_id(),
        user_id: 1,
        expires: Utc::now().timestamp()
    }
}

fn mock_cookie_jar() -> CookieJar {
    CookieJar::new().add(Cookie::new(SESSION_COOKIE_NAME.as_str(), mock_session_id()))
}

#[tokio::test]
async fn post_sessions_normal() {
    let mut session_service = MockSessionService::new();

    let session_data = mock_session_data();
    let session_data_cpy = session_data.clone();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .return_once(|_| Ok(session_data_cpy));

    let (jar, status) = post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.unwrap();
    assert_eq!(StatusCode::CREATED, status);

    let cookie = jar.get(SESSION_COOKIE_NAME.as_str()).unwrap();
    assert_eq!(session_data.id, cookie.value());
    assert_eq!(session_data.expires, cookie.expires().unwrap().datetime().unwrap().unix_timestamp());
    assert!(cookie.http_only().unwrap());
}

#[tokio::test]
async fn post_sessions_no_user_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(LoginError::NoUser));

    assert_eq!(StatusCode::UNAUTHORIZED, post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.err().unwrap())
}

#[tokio::test]
async fn post_sessions_unknown_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_login()
        .with(predicate::eq(mock_credentials()))
        .times(1)
        .returning(|_| Err(LoginError::Unknown));

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, post_sessions(Extension(session_service), CookieJar::new(), Json(mock_credentials())).await.err().unwrap())
}

#[tokio::test]
async fn get_sessions_normal() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_verify()
        .with(predicate::eq(mock_session_id()))
        .times(1)
        .returning(|_| Ok(mock_user()));

    let headers = get_sessions(Extension(session_service), mock_cookie_jar()).await.unwrap();
    assert_eq!((USER_ID_HEADER.as_str(), 1), headers.0[0]);
}

#[tokio::test]
async fn get_sessions_missing_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_verify()
        .with(predicate::eq(mock_session_id()))
        .times(1)
        .returning(|_| Err(SessionVerifyError::Missing));

    let res = get_sessions(Extension(session_service), mock_cookie_jar()).await.err().unwrap();
    assert_eq!(StatusCode::UNAUTHORIZED, res);
}

#[tokio::test]
async fn get_sessions_unknown_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_verify()
        .with(predicate::eq(mock_session_id()))
        .times(1)
        .returning(|_| Err(SessionVerifyError::Unknown));

    let res = get_sessions(Extension(session_service), mock_cookie_jar()).await.err().unwrap();
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, res);
}

#[tokio::test]
async fn delete_sessions_normal() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_logout()
        .with(predicate::eq(mock_session_id()))
        .times(1)
        .returning(|_| Ok(()));

    assert_eq!(StatusCode::OK, delete_sessions(Extension(session_service), mock_cookie_jar()).await);
}

#[tokio::test]
async fn delete_sessions_unknown_error() {
    let mut session_service = MockSessionService::new();

    session_service
        .expect_logout()
        .with(predicate::eq(mock_session_id()))
        .times(1)
        .returning(|_| Err(LogoutError::Unknown));

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, delete_sessions(Extension(session_service), mock_cookie_jar()).await);
}
