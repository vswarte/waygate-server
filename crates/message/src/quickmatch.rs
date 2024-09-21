use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSearchQuickMatchParams {
    pub quickmatch_settings: i32,
    pub unk1: u32,
    pub arena_id: i32,
    pub unk2: u32,
    pub matching_parameters: MatchingParameters,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParamsEntry {
    pub host_player_id: i32,
    pub host_steam_id: String,
    pub arena_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParams {
    pub matches: Vec<ResponseSearchQuickMatchParamsEntry>,
    pub unk1: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRegisterQuickMatchParams {
    pub quickmatch_settings: i32,
    pub arena_id: i32,
    pub matching_parameters: MatchingParameters,
    pub unk1: u8,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestJoinQuickMatchParams {
    pub unk1: u32,
    pub host_player_id: i32,
    pub joining_player_id: i32,
    pub arena_id: i32,
    pub unk2: u8,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAcceptQuickMatchParams {
    pub unk1: u32,
    pub joining_player_id: i32,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u32)]
pub enum QuickmatchResult {
    Win = 0,
    Lose = 1,
    Draw = 2,
    Error = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateBattleSessionParams {
    pub quickmatch_settings: u32,
    pub unk2: u32,
    // 0, 1, 3 but 3 seems to be some indicator of an error?
    pub result: QuickmatchResult,
    // Amount of kills
    pub eliminations: u8,
    // Looks like some form of unique ID
    pub unk5: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateBattleSessionParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}
