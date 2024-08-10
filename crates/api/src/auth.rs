use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::HeaderValue,
    Error
};
use waygate_config::GENERAL;
use std::future::{ready, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct CheckKey;

impl<S, B> Transform<S, ServiceRequest> for CheckKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CheckKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckKeyMiddleware { service }))
    }
}

pub struct CheckKeyMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let key = GENERAL.get().unwrap().api_key.as_str();
        let expected = HeaderValue::from_str(key).unwrap();
        let authorized = req.headers()
            .get("X-Auth-Token")
            .map(|v| v == expected)
            .unwrap_or_default();

        let fut = self.service.call(req);
        Box::pin(async move {
            if authorized {
                fut.await
            } else {
                Err(ErrorUnauthorized("Unauthorized"))
            }
        })
    }
}
