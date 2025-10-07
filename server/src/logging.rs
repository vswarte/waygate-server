use std::{cell::RefCell, collections::HashMap, future::Future};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures_util::future::{ready, LocalBoxFuture, Ready};
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::rc::Rc;
use tokio::task::futures::TaskLocalFuture;

tokio::task_local! {
    /// A task-local context for logging.
    static LOG_CONTEXT: LogContext;
}

#[derive(Clone)]
pub struct LogContext {
    context: RefCell<HashMap<String, String>>,
}

impl LogContext {
    fn new() -> Self {
        Self {
            context: RefCell::new(HashMap::new()),
        }
    }

    pub fn with<F>(f: F) -> TaskLocalFuture<LogContext, F>
    where
        F: Future,
    {
        LOG_CONTEXT.scope(LogContext::new(), f)
    }

    pub fn insert(key: impl Into<String>, value: impl Into<String>) {
        LOG_CONTEXT.with(|ctx| {
            ctx.context.borrow_mut().insert(key.into(), value.into());
        });
    }

    #[allow(dead_code)]
    pub fn get(key: &str) -> Option<String> {
        LOG_CONTEXT.with(|ctx| ctx.context.borrow().get(key).cloned())
    }

    #[allow(dead_code)]
    pub fn iter<F, R>(f: F) -> R
    where
        F: FnOnce(&HashMap<String, String>) -> R,
    {
        LOG_CONTEXT.with(|ctx| f(&ctx.context.borrow()))
    }

    pub fn current() -> LogContext {
        LOG_CONTEXT.with(|ctx| (*ctx).clone())
    }
}

impl Serialize for LogContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.context.borrow().len()))?;
        for (key, value) in self.context.borrow().iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

pub struct LogContextMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LogContextMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LogContextMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LogContextMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct LogContextMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LogContextMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);
        let fut = srv.call(req);

        Box::pin(async move {
            LogContext::with(async move {
                let res = fut.await?;
                Ok(res)
            })
            .await
        })
    }
}
