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

impl Into<ResponseGetAnnounceMessageListParamsEntry> for &AnnouncementConfigItem {
    fn into(self) -> ResponseGetAnnounceMessageListParamsEntry {
        ResponseGetAnnounceMessageListParamsEntry {
            index: self.index,
            order: self.order,
            unk1: 1,
            title: self.title.clone(),
            body: self.body.clone(),
            published_at: self.published_at,
        }
    }
}
