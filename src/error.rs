use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing query string")]
    MissingQueryString,
    #[error("Invalid query string")]
    InvalidQueryString,
    #[error("Hash doesn't match")]
    HashDoesntMatch,
    #[error("Missing user field")]
    MissingUser,
    #[error("Invalid user JSON")]
    InvalidUserJson,
}

impl From<AuthError> for Response {
    fn from(e: AuthError) -> Self {
        e.into_response()
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::error!("Authorization error: {:?}", self);
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}
