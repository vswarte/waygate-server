use serde::{Deserialize, Serialize};

use super::MatchingParameters;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetMatchDensityParams {
    pub matching_parameters: MatchingParameters,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetMatchDensityParams {
    pub areas: Vec<i32>,
    pub blue_activity: Vec<u8>,
    pub red_activity: Vec<u8>,
    pub unk1: u32,
}
