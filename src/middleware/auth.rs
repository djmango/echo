use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::AUTHORIZATION,
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::{
    future::{ready, Ready},
    sync::Arc,
};
use tracing::{debug, warn};

use crate::{types::Claims, AppConfig};

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        matches!(
            self.user_id.as_str(),
            "user_01HRBJ8FVP3JT28DEWXN6JPKF5" | // Sulaiman skghori
            "user_01HY5EW9Z5XVE34GZXKH4NC2Y1" |
            "user_01J12R88378H1Z5R3JCGEPJ6RA"
        )
    }
}
// This is the trait that actix-web uses to extract the `AuthenticatedUser` from the request
// This is how we can use `AuthenticatedUser` as a parameter in our route handlers
// It automatically returns a 401 Unauthorized if the user is not authenticated
impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<AuthenticatedUser, Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(auth_user) = req.extensions().get::<AuthenticatedUser>() {
            ready(Ok(auth_user.clone())) // Assuming `AuthenticatedUser` can be cheaply cloned
        } else {
            ready(Err(ErrorUnauthorized("User not authenticated")))
        }
    }
}

pub struct AuthenticationMiddleware {
    pub app_config: Arc<AppConfig>,
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddlewareService {
            service,
            app_config: self.app_config.clone(),
        }))
    }
}

pub struct AuthenticationMiddlewareService<S> {
    service: S,
    app_config: Arc<AppConfig>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
    
        req.extensions_mut().insert(AuthenticatedUser { user_id: "me".to_string() });

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
