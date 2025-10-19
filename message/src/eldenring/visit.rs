use serde::{Deserialize, Serialize};

use super::*;

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum VisitType {
    /// Regular blue hunter helping host defeat invaders
    Hunter = 0,
    /// Hunter that invades "sinners"
    SinnerHunter = 1,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestVisitParams {
    pub unk1: u32,
    pub play_region: u32,
    pub visit_type: VisitType,
    pub player_id: i32,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseVisitParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetVisitorListParams {
    pub play_region: u32,
    pub max_count: u32,
    pub matching_parameters: MatchingParameters,
    pub visit_type: VisitType,
    pub unk3: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetVisitorListParamsEntry {
    pub player_id: i32,
    pub external_id: String,
    pub unk1: u32,
    pub play_region: u32,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetVisitorListParams {
    pub entries: Vec<ResponseGetVisitorListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectVisitParams {
    pub host_player_id: i32,
    pub visit_type: VisitType,
    pub play_region: i32,
    pub unk4: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRejectVisitParams {}
