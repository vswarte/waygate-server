use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRegisterUGCParams {
    pub unk1: u32,
    pub unk2: u32,
}
