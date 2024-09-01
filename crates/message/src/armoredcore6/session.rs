use serde::{Deserialize, Serialize};

use crate::ObjectIdentifier;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateSessionParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub game_version: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub steam_ticket: Vec<u8>,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionData {
    pub identifier: ObjectIdentifier,
    pub valid_from: i64,
    pub valid_until: i64,
    pub cookie: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateSessionParams {
    pub player_id: i32,
    pub steam_id: String,
    pub ip_address: String,
    pub session_data: SessionData,
    pub redirect_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRestoreSessionParams {
    pub game_version: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub steam_ticket: Vec<u8>,
    pub session_data: SessionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRestoreSessionParams {
    pub session_data: SessionData,
    pub unk_string: String,
}
