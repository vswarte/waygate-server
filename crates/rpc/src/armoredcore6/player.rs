use waygate_message::armoredcore6::{RequestGetRankingOrderParams, ResponseGetRankingOrderParams, ResponseGetRankingOrderParamsEntry, ResponseGetRatingStatusParams, ResponseParams};

use crate::HandlerResult;

pub async fn handle_get_rating_status() -> HandlerResult {
    Ok(ResponseParams::GetRatingStatus(
        ResponseGetRatingStatusParams {
            unk1: 5,
            unk2: 0,
            unk3: 5,
            unk4: 200,
            unk5: 400,
            unk6: 600,
            unk7: 1000,
            unk8: 1400,
            unk9: 0,
            unk10: 1500,
            unk11: 5,
            unk12: 0,
            unk13: 0,
            unk14: 0,
            unk15: 5,
            unk16: 0,
            unk17: 5,
            unk18: 200,
            unk19: 400,
            unk20: 600,
            unk21: 1000,
            unk22: 1400,
            unk23: 0,
            unk24: 1500,
            unk25: 5,
            unk26: 0,
            unk27: 0,
            unk28: 0,
            unk29: 0,
            unk30: 0,
            unk31: 0,
            unk32: 0,
            unk33: 0,
            unk34: 0,
            unk35: 0,
            unk36: 0,
            unk37: 0,
        },
    ))
}

pub async fn handle_get_ranking_order(
    request: Box<RequestGetRankingOrderParams>,
) -> HandlerResult {
    Ok(ResponseParams::GetRankingOrder(
        ResponseGetRankingOrderParams {
            entries: vec![
                ResponseGetRankingOrderParamsEntry {
                    player_id: u32::MAX,
                    player_name: String::from("TEST MAN"),
                    ac_name: String::from("TEST AC NAME"),
                    ac_ugc_code: String::from("TEXYWH9UEEVA"),
                    steam_id: String::from(""),
                    ranking: 1,
                    rating: 9999,
                },
            ],
        }
    ))
}
