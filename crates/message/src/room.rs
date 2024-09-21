use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateRoomParams {
    pub unk1: u32,
    pub unk2: Vec<u8>,
}

