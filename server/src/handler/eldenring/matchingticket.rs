use message::eldenring::{
    RequestCreateMatchingTicketParams, RequestDeleteMatchingTicket,
    RequestPollMatchingTicketParams, ResponseCreateMatchingTicketParams,
    ResponseDeleteMatchingTicketParams, ResponsePollMatchingTicketParams,
};

use crate::handler::HandleRequest;

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestPollMatchingTicketParams>, ResponsePollMatchingTicketParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestPollMatchingTicketParams>,
    ) -> Result<ResponsePollMatchingTicketParams, Box<dyn std::error::Error>> {
        Ok(ResponsePollMatchingTicketParams { unk0: 0 })
    }
}

impl HandleRequest<Box<RequestCreateMatchingTicketParams>, ResponseCreateMatchingTicketParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestCreateMatchingTicketParams>,
    ) -> Result<ResponseCreateMatchingTicketParams, Box<dyn std::error::Error>> {
        Ok(ResponseCreateMatchingTicketParams {})
    }
}

impl HandleRequest<Box<RequestDeleteMatchingTicket>, ResponseDeleteMatchingTicketParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestDeleteMatchingTicket>,
    ) -> Result<ResponseDeleteMatchingTicketParams, Box<dyn std::error::Error>> {
        Ok(ResponseDeleteMatchingTicketParams {})
    }
}
