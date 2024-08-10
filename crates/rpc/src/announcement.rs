use waygate_config::ANNOUNCEMENTS;
use waygate_message::*;

use crate::HandlerResult;

pub async fn handle_get_announce_message_list() -> HandlerResult {
    Ok(ResponseParams::GetAnnounceMessageList(
        ResponseGetAnnounceMessageListParams {
            list1: ANNOUNCEMENTS.get()
                .unwrap()
                .announcements
                .iter()
                .map(|i| ResponseGetAnnounceMessageListParamsEntry {
                    index: i.index,
                    order: i.order,
                    unk1: 1,
                    title: i.title.clone(),
                    body: i.body.clone(),
                    published_at: i.published_at,
                })
                .collect(),
            list2: vec![],
        }
    ))
}
