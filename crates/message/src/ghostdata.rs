use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateGhostDataParams {
    pub area: PlayRegionArea,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateGhostDataParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetGhostDataListParams {
    pub search_areas: Vec<PlayRegionArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetGhostDataListParamsEntry {
    pub area: PlayRegionArea,
    pub identifier: ObjectIdentifier,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetGhostDataListParams {
    pub entries: Vec<ResponseGetGhostDataListParamsEntry>,
}
