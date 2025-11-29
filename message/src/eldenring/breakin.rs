use serde::{Deserialize, Serialize};

use super::*;

/// Sent by invader looking for invadeable hosts.
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBreakInTargetListParams {
    /// Region in which we're searching for invasions.
    pub play_region: u32,
    /// Max amount of results in the response.
    pub max_count: u32,
    /// Player stats used for matching to other playrs.
    pub matching_parameters: MatchingParameters,
    pub unk48: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParams {
    pub play_region: u32,
    pub entries: Vec<ResponseGetBreakInTargetListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBreakInTargetListParamsEntry {
    pub player_id: i32,
    pub external_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBreakInTargetParams {
    pub unk0: u32,
    pub play_region: u32,
    pub player_id: i32,
    pub unkc: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBreakInTargetParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAllowBreakInTargetParams {
    pub invading_player_id: i32,
    pub join_data: Vec<u8>,
    pub unk1: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseAllowBreakInTargetParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectBreakInTargetParams {
    pub invading_player_id: i32,
    pub reason: i32,
    pub play_region: u32,
    pub unkc: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRejectBreakInTargetParams {}
