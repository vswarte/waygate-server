use serde::{Serialize, Deserialize};

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
    CreateSign(sign::ResponseCreateSignParams),
    CreateMatchAreaSign(sign::ResponseCreateMatchAreaSignParams),
    UpdateSign,
    GetSignList(sign::ResponseGetSignListParams),
    GetMatchAreaSignList(sign::ResponseGetMatchAreaSignListParams),
    RemoveSign,
    SummonSign,
    RejectSign,
    CreateBloodMessage(bloodmessage::ResponseCreateBloodMessageParams),
    RemoveBloodMessage,
    ReentryBloodMessage(bloodmessage::ResponseReentryBloodMessageParams),
    GetBloodMessageList(bloodmessage::ResponseGetBloodMessageListParams),
    EvaluateBloodMessage,
    GetBloodMessageDetail,
    CreateBloodstain(bloodstain::ResponseCreateBloodstainParams),
    GetBloodstainList(bloodstain::ResponseGetBloodstainListParams),
    GetDeadingGhost(bloodstain::ResponseGetDeadingGhostParams),
    CreateGhostData(ghostdata::ResponseCreateGhostDataParams),
    GetGhostDataList(ghostdata::ResponseGetGhostDataListParams),
    UpdateLoginPlayerCharacter(character::ResponseUpdateLoginPlayerCharacterParams),
    UpdatePlayerStatus,
    BreakInTarget,
    GetBreakInTargetList(breakin::ResponseGetBreakInTargetListParams),
    RejectBreakInTarget,
    AllowBreakInTarget,
    Visit,
    GetVisitorList,
    RejectVisit,
    NotifyAreaEvent,
    JoinMultiplay,
    LeaveMultiplay,
    GetMatchDensity,
    GetPlayZoneIdList,
    RegisterCharacterLog,
    SelectCharacterLog,
    DieLog,
    UseMagicLog,
    UseGestureLog,
    UseItemLog,
    PurchaseItemLog,
    GetItemLog,
    DropItemLog,
    LeaveItemLog,
    SaleItemLog,
    CreateItemLog,
    SummonBuddyLog,
    KillEnemyLog,
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
    SearchQuickMatch(quickmatch::ResponseSearchQuickMatchParams),
    RegisterQuickMatch,
    UnregisterQuickMatch,
    UpdateQuickMatch,
    JoinQuickMatch,
    AcceptQuickMatch,
    RejectQuickMatch,
    GrUploadPlayerEquipments,
    GrGetPlayerEquipments(player_equipments::ResponseGrGetPlayerEquipmentsParams),
    CreateMatchingTicket,
    PollMatchingTicket(matchingticket::ResponsePollMatchingTicketParams),
    DeleteMatchingTicket,
    CreateBattleSession(quickmatch::ResponseCreateBattleSessionParams),
    CreateRoom,
    UpdateRoom,
    DeleteRoom,
    GetRoom,
    GetRoomList,
    RegisterUGC(ugc::ResponseRegisterUGCParams),
    GetUGCSNSCodeList,
    GetUGC,
    DeleteUGC,
    SendQuickMatchStart,
    SendQuickMatchResult,
}
