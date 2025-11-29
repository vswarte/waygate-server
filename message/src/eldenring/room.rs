use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateRoomParams {
    pub game_version: u32,
    pub unk2: Vec<u8>,
}
