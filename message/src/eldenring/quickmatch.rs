use serde::{Deserialize, Serialize};
use wire::ShiftJisString;

use super::*;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QuickMatchSettings(pub u32);

wire::repr_enum! {
    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum QuickmatchTeamSlot {
        Ally = 0,
        Enemy = 1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchSize {
    /// Special case for Duel.
    Duel,
    /// One player vs one player in brawl or team mode with no additional allies.
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
}

impl QuickMatchSettings {
    /// Whether or not this gamemode allows spirit ashes summoning.
    pub const fn spirit_ashes_allowed(self) -> bool {
        self.0 >= 10
    }
    /// Whether or not this gamemode is brawl mode.
    pub const fn is_brawl_mode(self) -> bool {
        matches!(self.0 % 10, 1..=3)
    }
    /// Whether or not this gamemode is team-based.
    pub const fn is_team_mode(self) -> bool {
        matches!(self.0 % 10, 4..=9)
    }
    /// Whether or not this gamemode uses password for match you with your allies.
    /// Compared to just being password protected lobby where password doesn't affect team composition.
    pub const fn is_allies_password_mode(self) -> bool {
        matches!(self.0 % 10, 7..=9)
    }
    pub const fn match_size(self) -> QuickMatchSize {
        match self.0 % 10 {
            1 | 4 | 7 => QuickMatchSize::OneVsOne,
            2 | 5 | 8 => QuickMatchSize::TwoVsTwo,
            3 | 6 | 9 => QuickMatchSize::ThreeVsThree,
            _ => QuickMatchSize::Duel,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSearchQuickMatchParams {
    pub quickmatch_settings: QuickMatchSettings,
    pub arenas: Vec<u32>,
    pub search_limit: u32,
    pub matching_parameters: MatchingParameters,
    pub character_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParamsEntry {
    pub host_player_id: i32,
    pub host_external_id: String,
    pub arena_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSearchQuickMatchParams {
    pub matches: Vec<ResponseSearchQuickMatchParamsEntry>,
    pub target_team: QuickmatchTeamSlot,
    pub is_allied_password_opponent_search: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRegisterQuickMatchParams {
    pub quickmatch_settings: QuickMatchSettings,
    pub arena_id: u32,
    pub matching_parameters: MatchingParameters,
    pub is_allied_password: bool,
    pub character_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRegisterQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUnregisterQuickMatchParams {
    pub settings: QuickMatchSettings,
    pub map: u32,
    pub is_allied_password: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUnregisterQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateQuickMatchParams {
    pub quickmatch_settings: QuickMatchSettings,
    pub arena_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUpdateQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestJoinQuickMatchParams {
    pub settings: QuickMatchSettings,
    pub host_player_id: i32,
    pub character_id: u32,
    pub arena_id: u32,
    pub team: QuickmatchTeamSlot,
    pub password: ShiftJisString,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseJoinQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestAcceptQuickMatchParams {
    pub settings: QuickMatchSettings,
    pub joining_player_id: i32,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseAcceptQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestRejectQuickMatchParams {
    pub current_settings: QuickMatchSettings,
    pub joiner_player_id: u32,
    pub reject_reason: u32,
    pub arena_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseRejectQuickMatchParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuickMatchParticipant {
    pub player_id: u32,
    pub character_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSendQuickMatchStartParams {
    pub quickmatch_settings: QuickMatchSettings,
    pub participants: Vec<QuickMatchParticipant>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSendQuickMatchStartParams {}

#[derive(Serialize, Deserialize, Debug)]
#[repr(u32)]
pub enum QuickmatchResult {
    Win = 0,
    Lose = 1,
    Draw = 2,
    Error = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestSendQuickMatchResultParams {
    pub quickmatch_settings: QuickMatchSettings,
    pub character_id: u32,
    pub result: QuickmatchResult,
    pub eliminations: u8,
    pub multi_session_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSendQuickMatchResultParams {
    pub united_combat_rank: u32,
    pub united_combat_points: u32,
    pub duel_rank: u32,
    pub duel_points: u32,
    pub spirit_ashes_rank: u32,
    pub spirit_ashes_points: u32,
}
