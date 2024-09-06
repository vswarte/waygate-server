use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRatingStatus {
    pub unk6: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetRankingOrderParams {
    // 0 = SINGLE, 1 = TEAM
    pub category: u32,
    pub unk2: u32,
    pub entry_limit: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRankingOrderParamsEntry {
    pub player_id: u32,
    pub player_name: String,
    pub ac_name: String,
    pub ac_ugc_code: String,
    pub steam_id: String,
    pub ranking: u32,
    pub rating: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRankingOrderParams {
    pub entries: Vec<ResponseGetRankingOrderParamsEntry>,
}
