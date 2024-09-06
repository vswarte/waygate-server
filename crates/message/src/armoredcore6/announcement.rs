use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetAnnounceMessageListParams {
    pub max_entries: u32,
    pub unk1: u32,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetAnnounceMessageListParamsEntry {
    pub index: u32,
    pub order: u32,
    pub unk1: u32,
    pub title: String,
    pub body: String,
    pub published_at: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetAnnounceMessageListParams {
    pub list1: Vec<ResponseGetAnnounceMessageListParamsEntry>,
    pub list2: Vec<ResponseGetAnnounceMessageListParamsEntry>,
}
