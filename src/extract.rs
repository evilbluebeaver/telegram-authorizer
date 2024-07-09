use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{error::AuthError, layer::User};

#[derive(Debug, Clone)]
pub struct TelegramUser(pub u64);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for TelegramUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(parts
            .extensions
            .get::<TelegramUser>()
            .ok_or(AuthError::MissingUser)?
            .clone())
    }
}

impl From<User> for TelegramUser {
    fn from(user: User) -> Self {
        TelegramUser(user.id)
    }
}
