use serde::{Deserialize, Serialize};
use std::fmt;
use wire::ShiftJisString;

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

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum SellRegion {
    None = 0,
    Asia = 1,
    Japan = 2,
    NorthAmerica = 3,
    UnitedKingdom = 4,
    Global = 5,
}

/// Player stats used for matching to other players.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MatchingParameters {
    /// Regulation version, used to be the game version.
    pub regulation_version: u32,
    /// Sell region
    pub sell_region: SellRegion,
    /// Indicates if cross-region matchmaking is disallowed.
    pub cross_region_matchmaking_disabled: bool,
    /// CSNetMan->0xC
    pub unk2: u32,
    /// Calls static method on PlatformNetworkMan, returns 0 for PC builds.
    pub platform: u8,
    /// Boring old character rune level.
    pub character_level: u32,
    /// NetPlayerWatcher->0x10, always 0 for PC builds.
    pub unk3: u32,
    /// NG cycle.
    pub game_clear_count: u32,
    /// Main password in the multiplayer settings menu.
    pub password: ShiftJisString,
    /// Yes its still in here.
    pub vow_type: u32,
    /// Maximum reinforcement level for matching.
    pub max_reinforce: u16,
    /// Maximum spirit ash reinforcement level for matching.
    pub max_spirit_ash_reinforce: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub map: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
