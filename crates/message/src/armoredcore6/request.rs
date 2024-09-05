use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug)]
#[repr(u32)]
pub enum RequestParams {
    CreateSession(Box<session::RequestCreateSessionParams>),
    DeleteSession,
    RestoreSession(Box<session::RequestRestoreSessionParams>),
    DebugCommand,
    ServerPing,
    CheckAlive,
    GetAnnounceMessageList(Box<announcement::RequestGetAnnounceMessageListParams>),
    UpdateLoginPlayerCharacter,
    SystemLog,
    MenuUsageLog,
    InGameResultLog,
    UploadFamilySharingInfo,
    UploadGuardITCode,
    CreateMatchingTicket(Box<matchingticket::RequestCreateMatchingTicketParams>),
    PollMatchingTicket,
    DeleteMatchingTicket,
    StartBattleSession,
    FinishBattleSession,
    CreateBattleSession,
    CreateRoom(Box<room::RequestCreateRoomParams>),
    UpdateRoom(Box<room::RequestUpdateRoomParams>),
    DeleteRoom(Box<room::RequestDeleteRoomParams>),
    GetRoom,
    GetRoomList(Box<RequestGetRoomListParams>),
    GenerateRoomBattleID(Box<room::RequestGenerateRoomBattleID>),
    RegisterUGC(Box<ugc::RequestRegisterUGCParams>),
    GetUGCIDList,
    GetUGC,
    DeleteUGC,
    GetUGCStatus(Box<ugc::RequestGetUGCStatusParams>),
    GenerateUGCServerID,
    KeepUGCServerID,
    UpdatePlayerProperties,
    GetPlayerProperties,
    GetRankingOrder(Box<player::RequestGetRankingOrder>),
    GetRatingStatus,
    GetPlayerRatingValue,
    Report,
    Healthcheck,
}

impl RequestParams {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CreateSession(_) => "CreateSession",
            Self::DeleteSession => "DeleteSession",
            Self::RestoreSession(_) => "RestoreSession",
            Self::DebugCommand => "DebugCommand",
            Self::ServerPing => "ServerPing",
            Self::CheckAlive => "CheckAlive",
            Self::GetAnnounceMessageList(_) => "GetAnnounceMessageList",
            Self::UpdateLoginPlayerCharacter => "UpdateLoginPlayerCharacter",
            Self::SystemLog => "SystemLog",
            Self::MenuUsageLog => "MenuUsageLog",
            Self::InGameResultLog => "InGameResultLog",
            Self::UploadFamilySharingInfo => "UploadFamilySharingInfo",
            Self::UploadGuardITCode => "UploadGuardITCode",
            Self::CreateMatchingTicket(_) => "CreateMatchingTicket",
            Self::PollMatchingTicket => "PollMatchingTicket",
            Self::DeleteMatchingTicket => "DeleteMatchingTicket",
            Self::StartBattleSession => "StartBattleSession",
            Self::FinishBattleSession => "FinishBattleSession",
            Self::CreateBattleSession => "CreateBattleSession",
            Self::CreateRoom(_) => "CreateRoom",
            Self::UpdateRoom(_) => "UpdateRoom",
            Self::DeleteRoom(_) => "DeleteRoom",
            Self::GetRoom => "GetRoom",
            Self::GetRoomList(_) => "GetRoomList",
            Self::GenerateRoomBattleID(_) => "GenerateRoomBattleID",
            Self::RegisterUGC(_) => "RegisterUGC",
            Self::GetUGCIDList => "GetUGCIDList",
            Self::GetUGC => "GetUGC",
            Self::DeleteUGC => "DeleteUGC",
            Self::GetUGCStatus(_) => "GetUGCStatus",
            Self::GenerateUGCServerID => "GenerateUGCServerID",
            Self::KeepUGCServerID => "KeepUGCServerID",
            Self::UpdatePlayerProperties => "UpdatePlayerProperties",
            Self::GetPlayerProperties => "GetPlayerProperties",
            Self::GetRankingOrder(_) => "GetRankingOrder",
            Self::GetRatingStatus => "GetRatingStatus",
            Self::GetPlayerRatingValue => "GetPlayerRatingValue",
            Self::Report => "Report",
            Self::Healthcheck => "Healthcheck",
        }
    }
}
