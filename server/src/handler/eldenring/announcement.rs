use crate::handler::HandleRequest;
use message::eldenring::{
    RequestGetAnnounceMessageListParams, ResponseGetAnnounceMessageListParams,
    ResponseGetAnnounceMessageListParamsEntry,
};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestGetAnnounceMessageListParams>, ResponseGetAnnounceMessageListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestGetAnnounceMessageListParams>,
    ) -> Result<ResponseGetAnnounceMessageListParams, Box<dyn std::error::Error>> {
        Ok(ResponseGetAnnounceMessageListParams {
            changes: vec![ResponseGetAnnounceMessageListParamsEntry {
                index: 0,
                order: 0,
                unk1: 0,
                title: String::from("Welcome to cum"),
                body: String::from("Yes, cum."),
                published_at: 0,
            }],
            notices: vec![],
        })
    }
}
