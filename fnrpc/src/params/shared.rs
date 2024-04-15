use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OnlineArea {
    pub map: i32,
    pub play_region: i32,
}

impl fmt::Debug for OnlineArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OnlineArea")
            .field(&self.map)
            .field(&self.play_region)
            .finish()
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct ObjectIdentifier {
    pub object_id: i32,

    // This seems to be either the player ID or the session ID but in the case
    // of push messages it seems to be random?
    // TODO: fact-check this (reference with all dumped buffers since it might
    // cause caching bugs on the client if handled poorly).
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
