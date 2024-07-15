use crate::rpc;
use crate::config;

use super::message::*;

pub async fn handle_get_announce_message_list() -> rpc::HandlerResult {
    Ok(ResponseParams::GetAnnounceMessageList(
        ResponseGetAnnounceMessageListParams {
            list1: config::get()
                .announcements
                .iter()
                .map(|i| i.into())
                .collect(),
            list2: vec![],
        }
    ))
}

impl From<&config::AnnouncementItem> for ResponseGetAnnounceMessageListParamsEntry {
    fn from(val: &config::AnnouncementItem) -> Self {
        ResponseGetAnnounceMessageListParamsEntry {
            index: val.index,
            order: val.order,
            unk1: 1,
            title: val.title.clone(),
            body: val.body.clone(),
            published_at: val.published_at,
        }
    } 
}
