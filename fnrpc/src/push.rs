use serde::Serialize;
use crate::params::*;

#[derive(Serialize, Debug)]
pub struct PushJoinUnk0Payload {
    pub unk1: u64,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnk1Payload {
    pub unk1: u32,
    pub unk2: u64,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct PushInvaderJoiningParams {
    pub invader_player_id: i32,
    pub invader_steam_id: String,
    pub unk1: u32,
    pub unk2: u32,
    pub play_region: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoiningAsInvaderParams {
    pub host_player_id: i32,
    pub join_data: Vec<u8>,
    pub unk1: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnk4Payload {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: String,
}

#[derive(Serialize, Debug)]
pub struct PushPlayerJoiningParams {
    pub summoned_player_id: u32,
    pub sign_identifier: shared::ObjectIdentifier,
}

#[derive(Serialize, Debug)]
pub struct PushJoiningPlayerParams {
    pub summoning_player_id: i32,
    pub steam_id: String,
    pub summoned_player_id: i32,
    pub sign_identifier: shared::ObjectIdentifier,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnk7Payload {
    pub unk1: u64,
    pub unk2: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnk8Payload {
    pub unk1: u32,
    pub unk2: String,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoiningAsBlueParams {
    pub host_player_id: u32,
    pub host_player_steam_id: String,
    pub join_data: Vec<u8>,
    pub unk1: u32,
    pub unk2: u32,
    pub play_region_id: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnkAPayload {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: String,
    pub unk4: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoiningQuickMatchParams {
    pub quickmatch_settings: i32,
    pub host_player_id: i32,
    pub host_steam_id: String,
    pub join_data: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct PushPlayerJoiningQuickMatchParams {
    pub quickmatch_settings: i32,
    pub joining_player_id: i32,
    pub joining_player_steam_id: String,
    pub unk2: i32,
    pub arena_id: i32,
    pub unk3: u8,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnkDPayload {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnkEPayload {
    pub unk1: u32,
    pub unk2: String,
    pub unk3: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct PushJoinUnkFPayload {
    pub unk1: Vec<u8>,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum JoinPayload {
    Unk0(PushJoinUnk0Payload), // 0x0
    Unk1(PushJoinUnk1Payload), // 0x1
    InvaderJoining(PushInvaderJoiningParams), // 0x2
    JoiningAsInvader(PushJoiningAsInvaderParams), // 0x3
    Unk4(PushJoinUnk4Payload), // 0x4
    PlayerJoining(PushPlayerJoiningParams), // 0x5
    JoiningPlayer(PushJoiningPlayerParams), //0x6
    Unk7(PushJoinUnk7Payload), // 0x7
    Unk8(PushJoinUnk8Payload), // 0x8
    JoiningAsBlue(PushJoiningAsBlueParams), // 0x9
    UnkA(PushJoinUnkAPayload), // 0XA
    PlayerJoiningQuickMatch(PushPlayerJoiningQuickMatchParams), // 0xB
    JoiningQuickMatch(PushJoiningQuickMatchParams), // 0xC
    UnkD(PushJoinUnkDPayload), // 0xD
    UnkE(PushJoinUnkEPayload), // 0xE
    UnkF(PushJoinUnkFPayload), // 0xF
}

#[derive(Serialize, Debug)]
pub struct PushJoinParams {
    pub identifier: shared::ObjectIdentifier,
    pub join_payload: JoinPayload,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum PushNotifyParamsSection1 {
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
pub enum PushNotifyParamsSection2 {
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
pub struct PushNotifyParams {
    pub identifier: shared::ObjectIdentifier,
    pub timestamp: u64,

    pub section1: PushNotifyParamsSection1,
    pub section2: PushNotifyParamsSection2,
}

#[derive(Serialize, Debug)]
#[repr(u32)]
pub enum PushParams {
    Notify(PushNotifyParams),
    Join(PushJoinParams),
}
