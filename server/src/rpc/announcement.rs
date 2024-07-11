use fnrpc::ResponseParams;
use fnrpc::announcement::*;

use crate::rpc;
use crate::config::{self, announcement::AnnouncementConfigItem};

pub async fn handle_get_announce_message_list() -> rpc::HandlerResult {
    Ok(ResponseParams::GetAnnounceMessageList(
        ResponseGetAnnounceMessageListParams {
            list1: config::announcement::get_config()
                .announcements
                .iter()
                .map(|i| i.into())
                .collect(),
            list2: vec![],
        }
    ))
}

impl From<&AnnouncementConfigItem> for ResponseGetAnnounceMessageListParamsEntry {
    fn from(val: &AnnouncementConfigItem) -> Self {
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
