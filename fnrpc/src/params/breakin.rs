use serde::{Serialize, Deserialize};

use crate::params::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBreakInTargetListParams {
    pub play_region: u32,
    pub unk2: u32,
    pub matching_parameters: MatchingParameters,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParamsEntry {
    pub player_id: i32,
    pub steam_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParams {
    pub unk1: u32,
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
    pub unk1: u32,
    pub unk2: u32,
    pub player_id: i32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectBreakInTargetParams {

}
