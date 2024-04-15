use serde::{Serialize, Deserialize};

pub const POOL_TYPE_FIA: u32 = 0x0;
pub const POOL_TYPE_JAR: u32 = 0x1;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGrGetPlayerEquipmentsParams {
    pub pool_type: u32,
    pub unk2: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGrGetPlayerEquipmentsParamsEntry {
    pub entry_id: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGrGetPlayerEquipmentsParams {
    pub entries: Vec<ResponseGrGetPlayerEquipmentsParamsEntry>,
}
