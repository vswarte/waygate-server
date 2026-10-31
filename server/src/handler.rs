pub mod eldenring;

use std::{error::Error, future::Future};

use message::eldenring::{NetError, ServerError};

pub(crate) struct InternalError(String);

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::fmt::Debug for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalError({:?})", self.0)
    }
}

impl Error for InternalError {}

impl NetError for InternalError {
    fn to_wire_code(&self) -> u32 {
        ServerError::Unknown as u32
    }
}

pub(crate) fn into_net_err(e: Box<dyn Error>) -> Box<dyn NetError> {
    match e.downcast::<ServerError>() {
        Ok(e) => e,
        Err(e) => Box::new(InternalError(e.to_string())),
    }
}

/// Handles a particular clients connection. Facilitates interaction with the rest of the servers
/// facilities.
pub trait RequestHandler<R, S> {
    fn dispatch_request(
        &mut self,
        request: &R,
    ) -> impl Future<Output = Result<Option<S>, Box<dyn NetError>>>;
}

/// Handler for a specific request dispatched type.
pub trait HandleRequest<R, S> {
    fn handle(&mut self, request: &R) -> impl Future<Output = Result<S, Box<dyn Error>>>;
}
