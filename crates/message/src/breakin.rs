use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBreakInTargetListParams {
    pub play_region: u32,
    pub unk4: u32,
    pub matching_parameters: MatchingParameters,
    pub unk48: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParamsEntry {
    pub player_id: i32,
    pub steam_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParams {
    pub play_region: u32,
    pub entries: Vec<ResponseGetBreakInTargetListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAllowBreakInTargetParams {
    pub player_id: i32,
    pub join_data: Vec<u8>,
    pub unk1: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBreakInTargetParams {
    pub unk0: u32,
    pub unk4: u32,
    pub player_id: i32,
    pub unkc: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectBreakInTargetParams {
    pub invading_player_id: i32,
    pub reason: i32,
    pub play_region: u32,
    pub unkc: u32,
}
