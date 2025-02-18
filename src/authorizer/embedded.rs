use crate::error::AuthError;

use super::{authorize, Authorizer, User};
use ring::hmac;
use serde::Deserialize;

#[derive(Clone)]
pub struct Embedded(hmac::Key);

#[derive(Debug, Deserialize)]
struct EmbeddedUser {
    user: String,
}

impl TryFrom<EmbeddedUser> for User {
    type Error = serde_json::Error;
    fn try_from(value: EmbeddedUser) -> Result<Self, Self::Error> {
        let user = serde_json::from_str(&value.user)?;
        Ok(user)
    }
}

impl Embedded {
    pub fn new(bot_token: &str) -> Self {
        let key = hmac::Key::new(hmac::HMAC_SHA256, b"WebAppData");
        let tag = hmac::sign(&key, bot_token.as_bytes());
        Self(hmac::Key::new(hmac::HMAC_SHA256, tag.as_ref()))
    }
}

impl Authorizer for Embedded {
    fn authorize(&self, query_string: Option<&str>) -> Result<User, AuthError> {
        let embedder_user: EmbeddedUser = authorize(query_string, &self.0)?;
        serde_json::from_str(&embedder_user.user).map_err(|_| AuthError::InvalidUserJson)
    }
}
