use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PlayRegionArea {
    pub play_region: i32,
    pub area: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct PuddleArea {
    pub puddle_id: i32,
    pub area: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Copy)]
pub struct ObjectIdentifier(pub i64);

/// Player stats used for matching to other players.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct MatchingParameters {
    /// Regulation version, used to be the game version.
    pub regulation_version: u32,
    /// Some result from a lookup table based on the region.
    pub unk_region_flag: u32,
    /// Indicates if cross-region matchmaking is disallowed.
    pub cross_region_matchmaking_disabled: u32,
    /// CSNetMan->0xC
    pub unk2: u8,
    /// Calls static method on PlatformNetworkMan, returns 0 for PC builds.
    pub platform: u8,
    /// Boring old character rune level.
    pub character_level: u16,
    /// NetPlayerWatcher->0x10
    pub unk3: u32,
    pub unk4: u32,
    /// NG cycle.
    pub game_clear_count: u16,
    /// Main password in the multiplayer settings menu.
    pub password: String,
    /// Yes its still in here.
    pub vow_type: u32,
    /// Maximum reinforcement level for matching.
    pub max_reinforce: u16,
    /// PlayerGameData->0xC7
    pub unk6: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub map: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
