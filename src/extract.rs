use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{authorizer::User, error::AuthError};

#[derive(Debug, Clone)]
pub struct TelegramUser {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl<S> FromRequestParts<S> for TelegramUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(parts
            .extensions
            .get::<User>()
            .ok_or(AuthError::MissingUser)?
            .clone()
            .into())
    }
}

impl From<User> for TelegramUser {
    fn from(user: User) -> Self {
        TelegramUser {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
        }
    }
}
#[cfg(feature = "aide")]
mod aide {
    use aide::OperationInput;
    impl OperationInput for super::TelegramUser {}
}
