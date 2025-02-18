use crate::error::AuthError;

use super::{authorize, Authorizer, User};
use ring::{digest, hmac};

#[derive(Clone)]
pub struct External(hmac::Key);

impl External {
    pub fn new(key: &str) -> Self {
        let digest = digest::digest(&digest::SHA256, key.as_bytes());
        Self(hmac::Key::new(hmac::HMAC_SHA256, digest.as_ref()))
    }
}

impl Authorizer for External {
    fn authorize(&self, query_string: Option<&str>) -> Result<User, AuthError> {
        authorize(query_string, &self.0)
    }
}
