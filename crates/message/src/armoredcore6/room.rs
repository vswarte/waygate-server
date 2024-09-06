use serde::{Serialize, Deserialize};

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum MatchFormat {
    #[default]
    All = 0,
    Single = 1,
    Team = 2,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub enum MemberRotation {
    #[default]
    All = 0,
    Unk1 = 1,
    Locked = 2,
    WinnersRotate = 3,
    LosersRotate = 4,
    Random = 5,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RoomDetails {
    pub keywords: String,
    pub player_capacity: u32, // 0 = ALL, Other values are the amount of players.
    pub match_format: MatchFormat,
    pub time_limit: u32, // 0 = ALL, other values are in minutes.
    pub map: u32, // 0 = ALL, 10000 = Random.
    pub member_rotation: MemberRotation,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGetRoomListParams {
    pub room_details: RoomDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRoomListParamsEntry {
    pub room_details: RoomDetails,
    pub players_in_room: u32,
    pub unk10: u32,
    pub room_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGetRoomListParams {
    pub entries: Vec<ResponseGetRoomListParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestCreateRoomParams {
    pub room_details: RoomDetails,
    pub room_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseCreateRoomParams {
    pub room_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateRoomParams {
    pub room_id: u32,
    pub room_details: RoomDetails,
    pub players_in_room: u32,
    pub unk2: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUpdateRoomParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestDeleteRoomParams {
    pub room_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDeleteRoomParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestGenerateRoomBattleIDParams {
    pub room_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseGenerateRoomBattleIDParams {
    pub unk1: u64,
}
