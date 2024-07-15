use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBloodstainParams {
    pub area: PlayRegionArea,
    pub advertisement_data: Vec<u8>,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateBloodstainParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBloodstainListParams {
    pub search_areas: Vec<PlayRegionArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodstainListParamsEntry {
    pub area: PlayRegionArea,
    pub identifier: ObjectIdentifier,
    pub advertisement_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodstainListParams {
    pub entries: Vec<ResponseGetBloodstainListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetDeadingGhostParams {
    pub area: PlayRegionArea,
    pub identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetDeadingGhostParams {
    pub unk0: i32,
    pub unk4: i32,
    pub identifier: ObjectIdentifier,
    pub replay_data: Vec<u8>,
}
