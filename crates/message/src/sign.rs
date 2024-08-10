use serde::{Serialize, Deserialize};

use super::shared::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateSignParams {
    pub area: PlayRegionArea,
    pub matching_parameters: MatchingParameters,
    pub unk0: u32, // Could be sign type although this is also embedded in the sign data itself
    pub data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateSignParams {
    pub identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub search_areas: Vec<PlayRegionArea>,
    pub matching_parameters: MatchingParameters,
    // TODO: rest
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetSignListParamsEntry {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub area: PlayRegionArea,
    pub data: Vec<u8>,
    pub steam_id: String,
    pub unk_string: String,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub entries: Vec<ResponseGetSignListParamsEntry>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSummonSignParams {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectSignParams {
    pub sign_identifier: ObjectIdentifier,
    pub summoning_player_id: i32,
    pub unk1: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRemoveSignParams {
    pub sign_identifier: ObjectIdentifier,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateSignParams {
    pub identifier: ObjectIdentifier,
    pub unk0: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetMatchAreaSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub unk1: u32,
    pub area: PuddleArea,
    pub unk2: u8,
    pub matching_parameters: MatchingParameters,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMatchAreaSignListParamsEntry {
    pub player_id: i32,
    pub identifier: ObjectIdentifier,
    pub area: PuddleArea,
    pub unk1: i32,
    pub data: Vec<u8>,
    pub steam_id: String,
    pub unk_string: String,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMatchAreaSignListParams {
    pub known_signs: Vec<ObjectIdentifier>,
    pub entries: Vec<ResponseGetMatchAreaSignListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateMatchAreaSignParams {
    pub area: PuddleArea,
    pub unk1: i32,
    pub matching_parameters: MatchingParameters,
    pub unk2: i32,
    pub data: Vec<u8>,
    pub group_passwords: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateMatchAreaSignParams {
    pub identifier: ObjectIdentifier,
}
