use std::{
    pin::Pin,
    task::{Context, Poll},
};

use actix_service::{Service, Transform};
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, error::ErrorUnauthorized, http::header, Error,
};
use futures::future::{ok, Ready};
use futures::Future;

use crate::token::decodetoken;

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = AuthMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let heade = req.headers().clone();
        let fut = self.service.call(req);

        if heade.contains_key(header::AUTHORIZATION) {
            let token = heade.get(header::AUTHORIZATION).unwrap();
            let token_str = token.to_str().unwrap();
            let result = decodetoken(token_str);

            match result {
                Ok(_) => Box::pin(async move {
                    let res = fut.await?;
                    println!("认证通过");
                    Ok(res)
                }),
                Err(e) => Box::pin(async move { Err(ErrorUnauthorized(e.to_string())) }),
            }
        } else {
            Box::pin(async move { Err(ErrorUnauthorized("认证失败")) })
        }
    }
}
