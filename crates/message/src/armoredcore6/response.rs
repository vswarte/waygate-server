use serde::{Deserialize, Serialize};

use super::*;

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseParams {
    CreateSession(session::ResponseCreateSessionParams),
    DeleteSession,
    RestoreSession(session::ResponseRestoreSessionParams),
    DebugCommand,
    ServerPing,
    CheckAlive,
    GetAnnounceMessageList(announcement::ResponseGetAnnounceMessageListParams),
    UpdateLoginPlayerCharacter(character::ResponseUpdateLoginPlayerCharacterParams),
    SystemLog,
    MenuUsageLog,
    InGameResultLog,
    UploadFamilySharingInfo,
    UploadGuardITCode,
    CreateMatchingTicket,
    PollMatchingTicket(matchingticket::ResponsePollMatchingTicketParams),
    DeleteMatchingTicket,
    StartBattleSession,
    FinishBattleSession,
    CreateBattleSession,
    CreateRoom(room::ResponseCreateRoomParams),
    UpdateRoom(room::ResponseUpdateRoomParams),
    DeleteRoom(room::ResponseDeleteRoomParams),
    GetRoom,
    GetRoomList(room::ResponseGetRoomListParams),
    GenerateRoomBattleID(room::ResponseGenerateRoomBattleIDParams),
    RegisterUGC(ugc::ResponseRegisterUGCParams),
    GetUGCIDList,
    GetUGC(ugc::ResponseGetUGCParams),
    DeleteUGC,
    GetUGCStatus(ugc::ResponseGetUGCStatusParams),
    GenerateUGCServerID,
    KeepUGCServerID,
    UpdatePlayerProperties,
    GetPlayerProperties,
    GetRankingOrder(player::ResponseGetRankingOrderParams),
    GetRatingStatus(rating::ResponseGetRatingStatusParams),
    GetPlayerRatingValue,
    Report,
    Healthcheck,
}
