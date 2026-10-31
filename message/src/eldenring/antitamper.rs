use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSendArxanInfoParams {
    pub unk1: Vec<u32>,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseSendArxanInfoParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestSendFamilySharingParams {
    pub unk0: bool,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseSendFamilySharingParams {
    pub unk0: String,
}
