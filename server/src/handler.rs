pub mod eldenring;

/// Handles a particular clients connection. Facilitates interaction with the rest of the servers
/// facilities.
pub trait RequestHandler<R, S> {
    async fn dispatch_request(
        &mut self,
        request: &R,
    ) -> Result<Option<S>, Box<dyn std::error::Error>>;
}

/// Handler for a specific request dispatched type.
pub trait HandleRequest<R, S> {
    async fn handle(&mut self, request: &R) -> Result<S, Box<dyn std::error::Error>>;
}
