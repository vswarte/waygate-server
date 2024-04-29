use serde::{Serialize, Deserialize};
use crate::params::*;

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
    CreateSign(Box<sign::RequestCreateSignParams>),
    CreateMatchAreaSign,
    UpdateSign(Box<sign::RequestUpdateSignParams>),
    GetSignList(Box<sign::RequestGetSignListParams>),
    GetMatchAreaSignList,
    RemoveSign(Box<sign::RequestRemoveSignParams>),
    SummonSign(Box<sign::RequestSummonSignParams>),
    RejectSign(Box<sign::RequestRejectSignParams>),
    CreateBloodMessage(Box<bloodmessage::RequestCreateBloodMessageParams>),
    RemoveBloodMessage(Box<bloodmessage::RequestRemoveBloodMessageParams>),
    ReentryBloodMessage(Box<bloodmessage::RequestReentryBloodMessageParams>),
    GetBloodMessageList(Box<bloodmessage::RequestGetBloodMessageListParams>),
    EvaluateBloodMessage(Box<bloodmessage::RequestEvaluateBloodMessageParams>),
    GetBloodMessageDetail, // Never seen this being sent
    CreateBloodstain(Box<bloodstain::RequestCreateBloodstainParams>),
    GetBloodstainList(Box<bloodstain::RequestGetBloodstainListParams>),
    GetDeadingGhost(Box<bloodstain::RequestGetDeadingGhostParams>),
    CreateGhostData(Box<ghostdata::RequestCreateGhostDataParams>),
    GetGhostDataList(Box<ghostdata::RequestGetGhostDataListParams>),
    UpdateLoginPlayerCharacter,
    UpdatePlayerStatus(Box<player::RequestUpdatePlayerStatusParams>),
    BreakInTarget(Box<breakin::RequestBreakInTargetParams>),
    GetBreakInTargetList(Box<breakin::RequestGetBreakInTargetListParams>),
    RejectBreakInTarget(Box<breakin::RequestRejectBreakInTargetParams>),
    AllowBreakInTarget(Box<breakin::RequestAllowBreakInTargetParams>),
    Visit,
    GetVisitorList,
    RejectVisit,
    NotifyAreaEvent,
    JoinMultiplay(Box<player::RequestJoinMultiplayParams>),
    LeaveMultiplay,
    GetMatchDensity,
    GetPlayZoneIdList,
    RegisterCharacterLog,
    SelectCharacterLog,
    DieLog,
    UseMagicLog,
    UseGestureLog,
    UseItemLog(Box<player::RequestUseItemLogParams>),
    PurchaseItemLog,
    GetItemLog(Box<player::RequestGetItemLogParams>),
    DropItemLog,
    LeaveItemLog,
    SaleItemLog,
    CreateItemLog,
    SummonBuddyLog,
    KillEnemyLog(Box<player::RequestKillEnemyLogParams>),
    KillBossLog,
    GlobalEventLog,
    DiscoverMapPointLog,
    JoinMultiplayLog,
    LeaveMultiplayLog,
    CreateSignResultLog,
    SummonSignResultLog,
    BreakInResultLog,
    VisitResultLog,
    QuickMatchResultLog,
    QuickMatchEndLog,
    SystemOptionLog,
    SearchQuickMatch(Box<quickmatch::RequestSearchQuickMatchParams>),
    RegisterQuickMatch(Box<quickmatch::RequestRegisterQuickMatchParams>),
    UnregisterQuickMatch,
    UpdateQuickMatch,
    JoinQuickMatch(Box<quickmatch::RequestJoinQuickMatchParams>),
    AcceptQuickMatch(Box<quickmatch::RequestAcceptQuickMatchParams>),
    RejectQuickMatch,
    GrUploadPlayerEquipments,
    GrGetPlayerEquipments(Box<player_equipments::RequestGrGetPlayerEquipmentsParams>),
    CreateMatchingTicket(Box<matchingticket::RequestCreateMatchingTicketParams>),
    PollMatchingTicket,
    DeleteMatchingTicket,
    CreateBattleSession,
    CreateRoom,
    UpdateRoom,
    DeleteRoom,
    GetRoom,
    GetRoomList,
    RegisterUGC,
    GetUGCSNSCodeList,
    GetUGC,
    DeleteUGC,
    SendQuickMatchStart,
    SendQuickMatchResult,
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
            Self::CreateSign(_) => "CreateSign",
            Self::CreateMatchAreaSign => "CreateMatchAreaSign",
            Self::UpdateSign(_) => "UpdateSign",
            Self::GetSignList(_) => "GetSignList",
            Self::GetMatchAreaSignList => "GetMatchAreaSignList",
            Self::RemoveSign(_) => "RemoveSign",
            Self::SummonSign(_) => "SummonSign",
            Self::RejectSign(_) => "RejectSign",
            Self::CreateBloodMessage(_) => "CreateBloodMessage",
            Self::RemoveBloodMessage(_) => "RemoveBloodMessage",
            Self::ReentryBloodMessage(_) => "ReentryBloodMessage",
            Self::GetBloodMessageList(_) => "GetBloodMessageList",
            Self::EvaluateBloodMessage(_) => "EvaluateBloodMessage",
            Self::GetBloodMessageDetail => "GetBloodMessageDetail",
            Self::CreateBloodstain(_) => "CreateBloodstain",
            Self::GetBloodstainList(_) => "GetBloodstainList",
            Self::GetDeadingGhost(_) => "GetDeadingGhost",
            Self::CreateGhostData(_) => "CreateGhostData",
            Self::GetGhostDataList(_) => "GetGhostDataList",
            Self::UpdateLoginPlayerCharacter => "UpdateLoginPlayerCharacter",
            Self::UpdatePlayerStatus(_) => "UpdatePlayerStatus",
            Self::BreakInTarget(_) => "BreakInTarget",
            Self::GetBreakInTargetList(_) => "GetBreakInTargetList",
            Self::RejectBreakInTarget(_) => "RejectBreakInTarget",
            Self::AllowBreakInTarget(_) => "AllowBreakInTarget",
            Self::Visit => "Visit",
            Self::GetVisitorList => "GetVisitorList",
            Self::RejectVisit => "RejectVisit",
            Self::NotifyAreaEvent => "NotifyAreaEvent",
            Self::JoinMultiplay(_) => "JoinMultiplay",
            Self::LeaveMultiplay => "LeaveMultiplay",
            Self::GetMatchDensity => "GetMatchDensity",
            Self::GetPlayZoneIdList => "GetPlayZoneIdList",
            Self::RegisterCharacterLog => "RegisterCharacterLog",
            Self::SelectCharacterLog => "SelectCharacterLog",
            Self::DieLog => "DieLog",
            Self::UseMagicLog => "UseMagicLog",
            Self::UseGestureLog => "UseGestureLog",
            Self::UseItemLog(_) => "UseItemLog",
            Self::PurchaseItemLog => "PurchaseItemLog",
            Self::GetItemLog(_) => "GetItemLog",
            Self::DropItemLog => "DropItemLog",
            Self::LeaveItemLog => "LeaveItemLog",
            Self::SaleItemLog => "SaleItemLog",
            Self::CreateItemLog => "CreateItemLog",
            Self::SummonBuddyLog => "SummonBuddyLog",
            Self::KillEnemyLog(_) => "KillEnemyLog",
            Self::KillBossLog => "KillBossLog",
            Self::GlobalEventLog => "GlobalEventLog",
            Self::DiscoverMapPointLog => "DiscoverMapPointLog",
            Self::JoinMultiplayLog => "JoinMultiplayLog",
            Self::LeaveMultiplayLog => "LeaveMultiplayLog",
            Self::CreateSignResultLog => "CreateSignResultLog",
            Self::SummonSignResultLog => "SummonSignResultLog",
            Self::BreakInResultLog => "BreakInResultLog",
            Self::VisitResultLog => "VisitResultLog",
            Self::QuickMatchResultLog => "QuickMatchResultLog",
            Self::QuickMatchEndLog => "QuickMatchEndLog",
            Self::SystemOptionLog => "SystemOptionLog",
            Self::SearchQuickMatch(_) => "SearchQuickMatch",
            Self::RegisterQuickMatch(_) => "RegisterQuickMatch",
            Self::UnregisterQuickMatch => "UnregisterQuickMatch",
            Self::UpdateQuickMatch => "UpdateQuickMatch",
            Self::JoinQuickMatch(_) => "JoinQuickMatch",
            Self::AcceptQuickMatch(_) => "AcceptQuickMatch",
            Self::RejectQuickMatch => "RejectQuickMatch",
            Self::GrUploadPlayerEquipments => "GrUploadPlayerEquipments",
            Self::GrGetPlayerEquipments(_) => "GrGetPlayerEquipments",
            Self::CreateMatchingTicket(_) => "CreateMatchingTicket",
            Self::PollMatchingTicket => "PollMatchingTicket",
            Self::DeleteMatchingTicket => "DeleteMatchingTicket",
            Self::CreateBattleSession => "CreateBattleSession",
            Self::CreateRoom => "CreateRoom",
            Self::UpdateRoom => "UpdateRoom",
            Self::DeleteRoom => "DeleteRoom",
            Self::GetRoom => "GetRoom",
            Self::GetRoomList => "GetRoomList",
            Self::RegisterUGC => "RegisterUGC",
            Self::GetUGCSNSCodeList => "GetUGCSNSCodeList",
            Self::GetUGC => "GetUGC",
            Self::DeleteUGC => "DeleteUGC",
            Self::SendQuickMatchStart => "SendQuickMatchStart",
            Self::SendQuickMatchResult => "SendQuickMatchResult",
        }
    }
}
