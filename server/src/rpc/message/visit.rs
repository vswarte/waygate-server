use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestVisitParams {
    pub unk1: u32,
    pub play_region: u32,
    pub unk2: u32,
    pub player_id: i32,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetVisitorListParams {
    pub play_region: u32,
    pub unk1: u32,
    pub matching_parameters: MatchingParameters,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetVisitorListParamsEntry {
    pub player_id: i32,
    pub steam_id: String,
    pub unk1: u32,
    pub play_region: u32,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetVisitorListParams {
    pub entries: Vec<ResponseGetVisitorListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectVisitParams {
    // TODO
}
