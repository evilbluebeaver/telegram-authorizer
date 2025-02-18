use std::{collections::BTreeMap, fmt::write};

use crate::error::AuthError;

mod embedded;
mod external;

pub use embedded::Embedded;
pub use external::External;
use ring::hmac;
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthorizationString<'a> {
    hash: &'a str,

    #[serde(flatten)]
    #[serde(borrow)]
    other_fields: BTreeMap<&'a str, String>,
}

impl AuthorizationString<'_> {
    pub fn check(&self, key: &hmac::Key) -> Result<(), AuthError> {
        let mut string_to_hash = String::new();
        for (name, value) in &self.other_fields {
            write(&mut string_to_hash, format_args!("{}={}\n", name, value))
                .expect("Failed to write");
        }
        // Removing the last newline character
        string_to_hash.pop();
        let computed_hash = hex::encode(hmac::sign(key, string_to_hash.as_bytes()).as_ref());
        if computed_hash != self.hash {
            return Err(AuthError::HashDoesntMatch);
        }
        Ok(())
    }
}

pub trait Authorizer: Clone {
    fn authorize(&self, query_string: Option<&str>) -> Result<User, AuthError>;
}

pub(super) fn authorize<T: DeserializeOwned>(
    query_string: Option<&str>,
    key: &hmac::Key,
) -> Result<T, AuthError> {
    let query_string = query_string.ok_or(AuthError::MissingQueryString)?;
    let authorization_string: AuthorizationString = parse_query_string(query_string)?;
    authorization_string.check(key)?;
    parse_query_string(query_string)
}

fn parse_query_string<'de, 'str: 'de, T: Deserialize<'de>>(str: &'str str) -> Result<T, AuthError> {
    serde_querystring::from_str(str, serde_querystring::ParseMode::UrlEncoded).map_err(|e| {
        tracing::error!("Failed to parse query string: {}", e);
        AuthError::InvalidQueryString
    })
}
