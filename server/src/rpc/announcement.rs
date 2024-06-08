use fnrpc::ResponseParams;
use fnrpc::announcement::*;

use crate::rpc;

pub async fn handle_get_announce_message_list(
) -> rpc::HandlerResult {
    Ok(ResponseParams::GetAnnounceMessageList(
        ResponseGetAnnounceMessageListParams {
            list1: vec![
                ResponseGetAnnounceMessageListParamsEntry {
                    index: 1,
                    order: 1,
                    unk1: 1,
                    title: String::from("[Welcome to online multiplayer world of BINGUSTAN]"),
                    body: "Bingus is pleased to welcome you to his country.\n".to_string(),
                    published_at: 1645681378,
                },
                ResponseGetAnnounceMessageListParamsEntry {
                    index: 2,
                    order: 2,
                    unk1: 1,
                    title: String::from("[Changelog 0.1.1]"),
                    body: concat!(
                        "- Reworked the p2p to improve latency\n",
                        "- Implemented client mod versioning\n",
                        "- Added sign puddles\n",
                        "- Fixed some stability issues with quickmatches\n",
                    ).to_string(),
                    published_at: 1645681378,
                },
                ResponseGetAnnounceMessageListParamsEntry {
                    index: 3,
                    order: 3,
                    unk1: 1,
                    title: String::from("[Changelog 0.1.0]"),
                    body: concat!(
                        "- Added changelog\n",
                        "- Moved away persistence from diesel to sqlx\n",
                        "- Made sign/invasion pooling work off of in-memory pools\n",
                        "- Made passwords work for signs\n",
                    ).to_string(),
                    published_at: 1645681378,
                },
            ],
            list2: vec![],
        }
    ))
}
