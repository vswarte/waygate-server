use std::{future::{ready, Ready}, pin::Pin, task::{Context, Poll}};

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, error::ErrorUnauthorized, http::header::HeaderValue};

pub struct CheckKey(String);

impl CheckKey {
    pub fn new<S: ToString>(key: S) -> Self {
        Self(key.to_string())
    }
}

impl<S, B> Transform<S, ServiceRequest> for CheckKey
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = CheckKeyMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let api_key = self.0.clone();
        ready(Ok(CheckKeyMiddleware { service, api_key }))
    }
}

pub struct CheckKeyMiddleware<S> {
    api_key: String,
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckKeyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let expected = HeaderValue::from_str(&self.api_key).unwrap();
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
