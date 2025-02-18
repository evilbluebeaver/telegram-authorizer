use std::{
    future::{ready, Future},
    pin::Pin,
    task::{Context, Poll},
};

use axum::{extract::Request, response::Response};
use tower_layer::Layer;
use tower_service::Service;

use crate::{
    authorizer::{Authorizer, User},
    error::AuthError,
    Embedded, External,
};

#[derive(Clone)]
pub struct AuthorizationLayer<A: Authorizer> {
    authorizer: A,
}

impl AuthorizationLayer<Embedded> {
    pub fn new_embedded(bot_token: &str) -> Self {
        Self {
            authorizer: Embedded::new(bot_token),
        }
    }
}

impl AuthorizationLayer<External> {
    pub fn new_external(bot_token: &str) -> Self {
        Self {
            authorizer: External::new(bot_token),
        }
    }
}

impl<S, A: Authorizer> Layer<S> for AuthorizationLayer<A> {
    type Service = AuthorizationService<S, A>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthorizationService {
            inner,
            authorizer: self.authorizer.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthorizationService<S, A: Authorizer> {
    inner: S,
    authorizer: A,
}

impl<S, A: Authorizer> AuthorizationService<S, A> {
    fn authorize(&self, query_string: Option<&str>) -> Result<User, AuthError> {
        self.authorizer.authorize(query_string)
    }
}

impl<S, A: Authorizer> Service<Request> for AuthorizationService<S, A>
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
        let query_string = req.uri().query();
        let user = self.authorize(query_string);
        match user {
            Ok(user) => {
                req.extensions_mut().insert(user);
                Box::pin(self.inner.call(req))
            }
            Err(e) => Box::pin(ready(Ok(e.into()))),
        }
    }
}
