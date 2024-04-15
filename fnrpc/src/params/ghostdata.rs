use serde::{Serialize, Deserialize};

use crate::params::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateGhostDataParams {
    pub area: OnlineArea,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateGhostDataParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetGhostDataListParams {
    pub search_areas: Vec<OnlineArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetGhostDataListParamsEntry {
    pub area: OnlineArea,
    pub identifier: ObjectIdentifier,
    pub replay_data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetGhostDataListParams {
    pub entries: Vec<ResponseGetGhostDataListParamsEntry>,
}
