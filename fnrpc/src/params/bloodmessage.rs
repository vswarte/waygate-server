use serde::{Serialize, Deserialize};

use crate::params::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBloodMessageParams {
    pub area: PlayRegionArea,
    pub character_id: i32,
    pub data: Vec<u8>,
    pub unk: i32,
    pub group_passwords: Vec<String>,
}

pub type ResponseCreateBloodMessageParams = ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetBloodMessageListParams {
    pub search_areas: Vec<PlayRegionArea>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodMessageListParamsEntry {
    pub player_id: i32,
    pub character_id: i32,
    pub identifier: ObjectIdentifier,
    pub rating_good: i32,
    pub rating_bad: i32,
    pub data: Vec<u8>,
    pub area: PlayRegionArea,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetBloodMessageListParams {
    pub entries: Vec<ResponseGetBloodMessageListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestEvaluateBloodMessageParams {
    pub identifier: ObjectIdentifier,
    pub rating: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestReentryBloodMessageParams {
    pub identifiers: Vec<ObjectIdentifier>,
    pub unk: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseReentryBloodMessageParams {
    pub identifiers: Vec<ObjectIdentifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRemoveBloodMessageParams {
    pub identifier: ObjectIdentifier,
}
