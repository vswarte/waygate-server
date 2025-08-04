use std::fs::File;

use crate::handler::HandleRequest;
use message::eldenring::{
    RequestGetAnnounceMessageListParams, ResponseGetAnnounceMessageListParams,
    ResponseGetAnnounceMessageListParamsEntry,
};
use serde::{Deserialize, Serialize};

use super::DefaultClientHandler;

impl HandleRequest<Box<RequestGetAnnounceMessageListParams>, ResponseGetAnnounceMessageListParams>
    for DefaultClientHandler<'_>
{
    async fn handle(
        &mut self,
        _request: &Box<RequestGetAnnounceMessageListParams>,
    ) -> Result<ResponseGetAnnounceMessageListParams, Box<dyn std::error::Error>> {
        let announcements = serde_yaml::from_reader::<_, AnnouncementConfig>(File::open(
            "config/announcement.yml",
        )?)?;

        Ok(ResponseGetAnnounceMessageListParams {
            changes: announcements
                .changes
                .iter()
                .map(ResponseGetAnnounceMessageListParamsEntry::from)
                .collect(),
            notices: announcements
                .notices
                .iter()
                .map(ResponseGetAnnounceMessageListParamsEntry::from)
                .collect(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct AnnouncementConfig {
    pub changes: Vec<AnnouncementItem>,
    pub notices: Vec<AnnouncementItem>,
}

#[derive(Deserialize, Serialize)]
pub struct AnnouncementItem {
    index: u32,
    published_at: u64,
    title: String,
    body: String,
}

impl From<&AnnouncementItem> for ResponseGetAnnounceMessageListParamsEntry {
    fn from(value: &AnnouncementItem) -> Self {
        ResponseGetAnnounceMessageListParamsEntry {
            index: value.index,
            unk1: 0,
            unk2: 0,
            title: value.title.clone().into(),
            body: value.body.clone().into(),
            published_at: value.published_at,
        }
    }
}
