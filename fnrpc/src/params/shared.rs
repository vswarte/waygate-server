use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayRegionArea {
    pub play_region: i32,
    pub area: i32,
}

impl fmt::Debug for PlayRegionArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OnlineArea")
            .field(&self.area)
            .field(&self.play_region)
            .finish()
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ObjectIdentifier {
    pub object_id: i32,
    pub secondary_id: i32,
}

impl fmt::Debug for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ObjectIdentifier")
            .field(&self.object_id)
            .field(&self.secondary_id)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct MatchingParameters {
    pub game_version: u32,
    pub unk1: u32,
    pub region_flags: u32,
    pub unk2: u16,
    pub soul_level: u16,
    pub unk3: u32,
    pub unk4: u32,
    // TODO: fact-check this
    pub clear_count: u16,
    pub password: String,
    pub unk5: u32,
    pub max_reinforce: u16,
    // TODO: fact-check this
    pub unk6: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub map: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
