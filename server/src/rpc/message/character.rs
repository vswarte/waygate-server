use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateLoginPlayerCharacterParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUpdateLoginPlayerCharacterParams {
    pub character_id: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}
