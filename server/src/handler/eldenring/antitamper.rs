use message::eldenring::{
    RequestSendArxanInfoParams, RequestSendFamilySharingParams, ResponseSendArxanInfoParams,
    ResponseSendFamilySharingParams,
};

use crate::handler::{eldenring::BannedClientHandler, HandleRequest};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestSendArxanInfoParams>, ResponseSendArxanInfoParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestSendArxanInfoParams>,
    ) -> Result<ResponseSendArxanInfoParams, Box<dyn std::error::Error>> {
        Ok(ResponseSendArxanInfoParams {})
    }
}

impl HandleRequest<Box<RequestSendFamilySharingParams>, ResponseSendFamilySharingParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestSendFamilySharingParams>,
    ) -> Result<ResponseSendFamilySharingParams, Box<dyn std::error::Error>> {
        Ok(ResponseSendFamilySharingParams {
            unk0: String::new(),
        })
    }
}

impl HandleRequest<Box<RequestSendFamilySharingParams>, ResponseSendFamilySharingParams>
    for BannedClientHandler
{
    async fn handle(
        &mut self,
        _request: &Box<RequestSendFamilySharingParams>,
    ) -> Result<ResponseSendFamilySharingParams, Box<dyn std::error::Error>> {
        Ok(ResponseSendFamilySharingParams {
            unk0: String::new(),
        })
    }
}
