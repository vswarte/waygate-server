use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRegisterUGCParams {
    pub unk1: u32,
    pub data: Vec<u8>,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRegisterUGCParams {
    pub ugc_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetUGCStatusParams {
    pub unk1: u32,
    pub ugc_codes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetUGCStatusParamsEntry {
    pub code: String,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetUGCStatusParams {
    pub entries: Vec<ResponseGetUGCStatusParamsEntry>,
}
