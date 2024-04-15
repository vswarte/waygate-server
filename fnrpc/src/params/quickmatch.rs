use serde::{Serialize, Deserialize};

use crate::params::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSearchQuickMatchParams {
    pub quickmatch_settings: i32,
    pub unk2: u32,
    pub arena_id: i32,
    pub unk4: u32,
    pub matching_parameters: MatchingParameters,
    pub character_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParamsEntry {
    pub host_player_id: i32,
    pub host_steam_id: String,
    pub arena_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParams {
    pub items: Vec<ResponseSearchQuickMatchParamsEntry>,
    pub unk1: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRegisterQuickMatchParams {
    pub quickmatch_settings: i32,
    pub arena_id: i32,
    pub matching_parameters: MatchingParameters,
    pub unk2: u8,
    pub character_id: u32,
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
