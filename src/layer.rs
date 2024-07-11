use std::{
    future::{ready, Future},
    pin::Pin,
    task::{Context, Poll},
};

use aws_lc_rs::hmac;
use axum::{extract::Request, response::Response};
use qstring::QString;
use serde::Deserialize;
use tower_layer::Layer;
use tower_service::Service;

use crate::error::AuthError;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: u64,
}

#[derive(Clone)]
pub struct AuthorizationLayer(pub String);

impl AuthorizationLayer {
    fn prepare_key(&self) -> hmac::Key {
        let key = hmac::Key::new(hmac::HMAC_SHA256, b"WebAppData");
        let tag = hmac::sign(&key, self.0.as_bytes());
        hmac::Key::new(hmac::HMAC_SHA256, tag.as_ref())
    }
}

impl<S> Layer<S> for AuthorizationLayer {
    type Service = AuthorizationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        let key = self.prepare_key();
        AuthorizationService { inner, key }
    }
}

#[derive(Clone)]
pub struct AuthorizationService<S> {
    inner: S,
    key: hmac::Key,
}

impl<S> AuthorizationService<S> {
    fn authorize(&self, query_string: Option<&str>) -> Result<User, AuthError> {
        let query_params =
            QString::from(query_string.ok_or(AuthError::MissingQueryString)?).into_pairs();
        self.check_hash(query_params)
    }
    fn check_hash(&self, mut query_params: Vec<(String, String)>) -> Result<User, AuthError> {
        let hash_index = query_params
            .iter()
            .position(|(name, _)| name == "hash")
            .ok_or(AuthError::MissingHash)?;
        let hash = query_params.swap_remove(hash_index).1;
        query_params.sort();
        let data_to_check = query_params
            .iter()
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<String>>()
            .join("\n");
        let computed_key = hex::encode(hmac::sign(&self.key, data_to_check.as_bytes()).as_ref());
        let authorized = computed_key == hash;
        if !authorized {
            return Err(AuthError::HashDoesntMatch);
        }
        let user = query_params
            .iter()
            .find(|(name, _)| name == "user")
            .ok_or(AuthError::MissingUser)
            .map(|(_, value)| value.as_ref())?;

        serde_json::from_str(user).map_err(|_| AuthError::InvalidUserJson)
    }
}

impl<S> Service<Request> for AuthorizationService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Response: From<AuthError> + Send,
    S::Future: Send,
    S::Error: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let user = self.authorize(req.uri().query());
        match user {
            Ok(user) => {
                req.extensions_mut().insert(user);
                Box::pin(self.inner.call(req))
            }
            Err(e) => Box::pin(ready(Ok(e.into()))),
        }
    }
}
