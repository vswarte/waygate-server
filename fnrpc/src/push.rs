use serde::Serialize;
use crate::params::*;

#[derive(Serialize, Debug)]
pub struct Unk0Params {
    pub unk1: u64,
}

#[derive(Serialize, Debug)]
pub struct Unk1Params {
    pub unk1: u32,
    pub unk2: u64,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct BreakInTargetParams {
    pub invader_player_id: i32,
    pub invader_steam_id: String,
    pub unk1: u32,
    pub unk2: u32,
    pub play_region: u32,
}

#[derive(Serialize, Debug)]
pub struct AllowBreakInTargetParams {
    pub host_player_id: i32,
    pub join_data: Vec<u8>,
    pub unk1: u32,
}

#[derive(Serialize, Debug)]
pub struct Unk4Params {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: String,
}

#[derive(Serialize, Debug)]
pub struct Unk5Params {
    pub summoned_player_id: i32,
    pub sign_identifier: shared::ObjectIdentifier,
}

#[derive(Serialize, Debug)]
pub struct SummonSignParams {
    pub summoning_player_id: i32,
    pub steam_id: String,
    pub summoned_player_id: i32,
    pub sign_identifier: shared::ObjectIdentifier,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct Unk7Params {
    pub unk1: u64,
    pub unk2: u32,
}

#[derive(Serialize, Debug)]
pub struct Unk8Params {
    pub unk1: u32,
    pub unk2: String,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct VisitParams {
    pub host_player_id: i32,
    pub host_player_steam_id: String,
    pub join_data: Vec<u8>,
    pub unk1: u32,
    pub unk2: u32,
    pub play_region_id: i32,
}

#[derive(Serialize, Debug)]
pub struct UnkAParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: String,
    pub unk4: u32,
}

#[derive(Serialize, Debug)]
pub struct JoinQuickMatchParams {
    pub quickmatch_settings: i32,
    pub joining_player_id: i32,
    pub joining_player_steam_id: String,
    pub unk2: i32,
    pub arena_id: i32,
    pub unk3: u8,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct AcceptQuickMatchParams {
    pub quickmatch_settings: i32,
    pub host_player_id: i32,
    pub host_steam_id: String,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct UnkDParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct UnkEParams {
    pub unk1: u32,
    pub unk2: String,
    pub unk3: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct UnkFParams {
    pub unk1: Vec<u8>,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum JoinPayload {
    Unk0(Unk0Params), // 0x0
    Unk1(Unk1Params), // 0x1
    BreakInTarget(BreakInTargetParams), // 0x2
    AllowBreakInTarget(AllowBreakInTargetParams), // 0x3
    Unk4(Unk4Params), // 0x4
    Unk5(Unk5Params), // 0x5
    SummonSign(SummonSignParams), //0x6
    Unk7(Unk7Params), // 0x7
    Unk8(Unk8Params), // 0x8
    Visit(VisitParams), // 0x9
    UnkA(UnkAParams), // 0XA
    JoinQuickMatch(JoinQuickMatchParams), // 0xB
    AcceptQuickMatch(AcceptQuickMatchParams), // 0xC
    UnkD(UnkDParams), // 0xD
    UnkE(UnkEParams), // 0xE
    UnkF(UnkFParams), // 0xF
}

#[derive(Serialize, Debug)]
pub struct JoinParams {
    pub identifier: shared::ObjectIdentifier,
    pub join_payload: JoinPayload,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum NotifyParamsSection1 {
    Variant1 {
        unk1: u32,
        unk2: u32,
    },
    Variant2 {
        unk1: u8,
        password: String,
    },
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum NotifyParamsSection2 {
    Variant1 {
        message: String,
        unk1: u64,
        unk2: u32,
        unk3: u32,
    },
    Variant2 {
        unk1: u32,
        password: String,
        player_data: Vec<u8>,
    },
}

#[derive(Serialize, Debug)]
pub struct NotifyParams {
    pub identifier: shared::ObjectIdentifier,
    pub timestamp: u64,

    pub section1: NotifyParamsSection1,
    pub section2: NotifyParamsSection2,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum PushParams {
    Notify(NotifyParams),
    Join(JoinParams),
}
