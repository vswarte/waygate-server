use serde::{Deserialize, Serialize};
use wire::ShiftJisString;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetAnnounceMessageListParams {
    pub max_count: u32,
    pub unk1: u32,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetAnnounceMessageListParamsEntry {
    pub index: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub title: ShiftJisString,
    pub body: ShiftJisString,
    pub published_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetAnnounceMessageListParams {
    pub changes: Vec<ResponseGetAnnounceMessageListParamsEntry>,
    pub notices: Vec<ResponseGetAnnounceMessageListParamsEntry>,
}
