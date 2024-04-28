enum StatusCode: u8  {
    Error = 0x0,
    Success = 0x1,
};

enum PayloadType :u8  {
    Request = 0x4,
    Response = 0x5,
    Push = 0x6,
    HeartBeat = 0x7,
};

enum ParamsType : u32 {
    CreateSession = 0x0,
    DeleteSession = 0x1,
    RestoreSession = 0x2,
    DebugCommand = 0x3,
    ServerPing = 0x4,
    CheckAlive = 0x5,
    GetAnnounceMessageList = 0x6,
    CreateSign = 0x7,
    CreateMatchAreaSign = 0x8,
    UpdateSign = 0x9,
    GetSignList = 0xA,
    GetMatchAreaSignList = 0xB,
    RemoveSign = 0xC,
    SummonSign = 0xD,
    RejectSign = 0xE,
    CreateBloodMessage = 0xF,
    RemoveBloodMessage = 0x10,
    ReentryBloodMessage = 0x11,
    GetBloodMessageList = 0x12,
    EvaluateBloodMessage = 0x13,
    GetBloodMessageDetail = 0x14,
    CreateBloodstain = 0x15,
    GetBloodstainList = 0x16,
    GetDeadingGhost = 0x17,
    CreateGhostData = 0x18,
    GetGhostDataList = 0x19,
    UpdateLoginPlayerCharacter = 0x1A,
    UpdatePlayerStatus = 0x1B,

    // Sent by invader to request an invasion with a particular host.
    BreakInTarget = 0x1C,
    // Requested by invader. Gives list of hot single hosts in your area.
    GetBreakInTargetList = 0x1D,
    // Sent by host when host is not accepting invaders
    RejectBreakInTarget = 0x1E,
    // Sent by host when host allows invader in. Contains lobby data.
    AllowBreakInTarget = 0x1F,

    // Sent by host when blue select. Contains lobby data.
    Visit = 0x20,
    // Sent by host when invaded + looking for blues
    GetVisitorList = 0x21,
    // Sent by the blue when summon is rejected.
    RejectVisit = 0x22,

    NotifyAreaEvent = 0x23,
    JoinMultiplay = 0x24,
    LeaveMultiplay = 0x25,
    GetMatchDensity = 0x26,
    GetPlayZoneIDList = 0x27,

    RegisterCharacterLog = 0x28,
    SelectCharacterLog = 0x29,
    DieLog = 0x2A,
    UseMagicLog = 0x2B,
    UseGestureLog = 0x2C,
    // Gets sent when some goods are used
    UseItemLog = 0x2D,
    PurchaseItemLog = 0x2E,
    GetItemLog = 0x2F,
    DropItemLog = 0x30,
    LeaveItemLog = 0x31,
    SaleItemLog = 0x32,
    CreateItemLog = 0x33,
    SummonBuddyLog = 0x34,
    KillEnemyLog = 0x35,
    KillBossLog = 0x36,
    GlobalEventLog = 0x37,
    DiscoverMapPointLog = 0x38,
    JoinMultiplayLog = 0x39,
    LeaveMultiplayLog = 0x3A,
    CreateSignResultLog = 0x3B,
    SummonSignResultLog = 0x3C,
    BreakInResultLog = 0x3D,
    VisitResultLog = 0x3E,
    QuickMatchResultLog = 0x3F,
    QuickMatchEndLog = 0x40,

    SystemOptionLog = 0x41,
    SearchQuickMatch = 0x42,
    RegisterQuickMatch = 0x43,
    UnregisterQuickMatch = 0x44,
    UpdateQuickMatch = 0x45,
    JoinQuickMatch = 0x46,
    AcceptQuickMatch = 0x47,
    RejectQuickMatch = 0x48,
    GRUploadPlayerEquipments = 0x49,
    GRGetPlayerEquipments = 0x4A,
    CreateMatchingTicket = 0x4B,
    PollMatchingTicket = 0x4C,
    DeleteMatchingTicket = 0x4D,
    CreateBattleSession = 0x4E,
    CreateRoom = 0x4F,
    UpdateRoom = 0x50,
    DeleteRoom = 0x51,
    GetRoom = 0x52,
    GetRoomList = 0x53,
    RegisterUGC = 0x54,
    GetUGCSnsCodeList = 0x55,
    GetUGC = 0x56,
    DeleteUGC = 0x57,
    SendQuickMatchStart = 0x58,
    SendQuickMatchResult = 0x59,
};

enum PushType : u32  {
    Notify = 0x0,
    Join = 0x1,
};

struct PayloadHeader {
    PayloadType payloadType;

    if (
        payloadType == PayloadType::Response ||
        payloadType == PayloadType::Request
    ) {
        u32 sequence;
    } else if (payloadType == PayloadType::Push) {
        u8 unk1;
    }

    if (payloadType == PayloadType::Response) {
        StatusCode statusCode;
        
        if (statusCode == StatusCode::Error) {
            u32 errorCode;
        }
    }
};

struct Payload {
    PayloadHeader header;

    if (header.payloadType == PayloadType::Push) {
        PushType pushType;

        if (pushType == PushType::Notify)
            PushNotifyParams params;
        else if (pushType == PushType::Join)
            PushJoinParams params;

    } else if (header.payloadType == PayloadType::Request) {
        ParamsType paramsType;
        
        if (paramsType == ParamsType::CreateSession)
            RequestCreateSessionParams params;
        else if (paramsType == ParamsType::RestoreSession)
            RequestRestoreSessionParams params;
        else if (paramsType == ParamsType::DeleteSession)
            RequestDeleteSessionParams params;
        else if (paramsType == ParamsType::GetAnnounceMessageList)
            RequestGetAnnounceMessageListParams params;
        else if (paramsType == ParamsType::GetMatchDensity)
            RequestGetMatchDensityParams params;
        else if (paramsType == ParamsType::GetItemLog)
            RequestGetItemLogParams params;
        else if (paramsType == ParamsType::UseItemLog)
            RequestUseItemLogParams params;
        else if (paramsType == ParamsType::KillEnemyLog)
            RequestKillEnemyLogParams params;
        else if (paramsType == ParamsType::RegisterUGC)
            RequestRegisterUGCParams params;
        else if (paramsType == ParamsType::CreateMatchingTicket)
            RequestCreateMatchingTicketParams params;
        else if (paramsType == ParamsType::UpdateLoginPlayerCharacter)
            RequestUpdateLoginPlayerCharacterParams params;
        else if (paramsType == ParamsType::GetBloodMessageList)
            RequestGetBloodMessageListParams params;
        else if (paramsType == ParamsType::CreateBloodMessage)
            RequestCreateBloodMessageParams params;
        else if (paramsType == ParamsType::ReentryBloodMessage)
            RequestReentryBloodMessageParams params;
        else if (paramsType == ParamsType::GetSignList)
            RequestGetSignListParams params;
        else if (paramsType == ParamsType::CreateGhostData)
            RequestCreateGhostDataParams params;
        else if (paramsType == ParamsType::GetGhostDataList)
            RequestGetGhostDataListParams params;
        else if (paramsType == ParamsType::GetBloodstainList)
            RequestGetBloodstainListParams params;
        else if (paramsType == ParamsType::CreateBloodstain)
            RequestCreateBloodstainParams params;
        else if (paramsType == ParamsType::CreateSign)
            RequestCreateSignParams params;
        else if (paramsType == ParamsType::CreateMatchAreaSign)
            RequestCreateMatchAreaSignParams params;
        else if (paramsType == ParamsType::GetDeadingGhost)
            RequestGetDeadingGhostParams params;
        else if (paramsType == ParamsType::EvaluateBloodMessage)
            RequestEvaluateBloodMessageParams params;
        else if (paramsType == ParamsType::SummonSign)
            RequestSummonSignParams params;
        else if (paramsType == ParamsType::GetMatchAreaSignList)
            RequestGetMatchAreaSignListParams params;
        else if (paramsType == ParamsType::RejectSign)
            RequestRejectSignParams params;
        else if (paramsType == ParamsType::RemoveSign)
            RequestRemoveSignParams params;
        else if (paramsType == ParamsType::SearchQuickMatch)
            RequestSearchQuickMatchParams params;
        else if (paramsType == ParamsType::JoinQuickMatch)
            RequestJoinQuickMatchParams params;
        else if (paramsType == ParamsType::UpdateQuickMatch)
            RequestUpdateQuickMatchParams params;
        else if (paramsType == ParamsType::RegisterQuickMatch)
            RequestRegisterQuickMatchParams params;
        else if (paramsType == ParamsType::AcceptQuickMatch)
            RequestAcceptQuickMatchParams params;
        else if (paramsType == ParamsType::UnregisterQuickMatch)
            RequestUnregisterQuickMatchParams params;
        else if (paramsType == ParamsType::GetBreakInTargetList)
            RequestGetBreakInTargetListParams params;
        else if (paramsType == ParamsType::AllowBreakInTarget)
            RequestAllowBreakInTargetParams params;
        else if (paramsType == ParamsType::GetVisitorList)
            RequestGetVisitorListParams params;
        else if (paramsType == ParamsType::Visit)
            RequestVisitParams params;
        else if (paramsType == ParamsType::UpdatePlayerStatus)
            RequestUpdatePlayerStatusParams params;
        else if (paramsType == ParamsType::BreakInTarget)
            RequestBreakInTargetParams params;
        else if (paramsType == ParamsType::GRGetPlayerEquipments)
            RequestGRGetPlayerEquipmentsParams params;
        else if (paramsType == ParamsType::GRUploadPlayerEquipments)
            RequestGRUploadPlayerEquipmentsParams params;
        else if (paramsType == ParamsType::JoinMultiplay)
            RequestJoinMultiplayParams params;
        else if (paramsType == ParamsType::JoinMultiplayLog)
            RequestJoinMultiplayLogParams params;

    } else if (
        header.payloadType == PayloadType::Response &&
        header.statusCode == StatusCode::Success
    ) {
        ParamsType paramsType;
        
        if (paramsType == ParamsType::CreateSession)
            ResponseCreateSessionParams params;
        else if (paramsType == ParamsType::DeleteSession)
            ResponseDeleteSessionParams params;
        else if (paramsType == ParamsType::RestoreSession)
            ResponseRestoreSessionParams params;
        else if (paramsType == ParamsType::GetAnnounceMessageList)
            ResponseGetAnnounceMessageListParams params;
        else if (paramsType == ParamsType::GetMatchDensity)
            ResponseGetMatchDensityParams params;
        else if (paramsType == ParamsType::GetItemLog)
            ResponseGetItemLogParams params;
        else if (paramsType == ParamsType::RegisterUGC)
            ResponseRegisterUGCParams params;
        else if (paramsType == ParamsType::CreateMatchingTicket)
            ResponseCreateMatchingTicketParams params;
        else if (paramsType == ParamsType::UpdateLoginPlayerCharacter)
            ResponseUpdateLoginPlayerCharacterParams params;
        else if (paramsType == ParamsType::GetBloodMessageList)
            ResponseGetBloodMessageListParams params;
        else if (paramsType == ParamsType::CreateBloodMessage)
            ResponseCreateBloodMessageParams params;
        else if (paramsType == ParamsType::ReentryBloodMessage)
            ResponseReentryBloodMessageParams params;
        else if (paramsType == ParamsType::GetSignList)
            ResponseGetSignListParams params;
        else if (paramsType == ParamsType::CreateGhostData)
            ResponseCreateGhostDataParams params;
        else if (paramsType == ParamsType::GetGhostDataList)
            ResponseGetGhostDataListParams params;
        else if (paramsType == ParamsType::GetBloodstainList)
            ResponseGetBloodstainListParams params;
        else if (paramsType == ParamsType::CreateBloodstain)
            ResponseCreateBloodstainParams params;
        else if (paramsType == ParamsType::CreateSign)
            ResponseCreateSignParams params;
        else if (paramsType == ParamsType::GetDeadingGhost)
            ResponseGetDeadingGhostParams params;
        else if (paramsType == ParamsType::EvaluateBloodMessage)
            ResponseEvaluateBloodMessageParams params;
        else if (paramsType == ParamsType::GetMatchAreaSignList)
            ResponseGetMatchAreaSignListParams params;
        else if (paramsType == ParamsType::SearchQuickMatch)
            ResponseSearchQuickMatchParams params;
        else if (paramsType == ParamsType::JoinQuickMatch)
            ResponseJoinQuickMatchParams params;
        else if (paramsType == ParamsType::UpdateQuickMatch)
            ResponseUpdateQuickMatchParams params;
        else if (paramsType == ParamsType::RegisterQuickMatch)
            ResponseRegisterQuickMatchParams params;
        else if (paramsType == ParamsType::AcceptQuickMatch)
            ResponseAcceptQuickMatchParams params;
        else if (paramsType == ParamsType::UnregisterQuickMatch)
            ResponseUnregisterQuickMatchParams params;
        else if (paramsType == ParamsType::GetBreakInTargetList)
            ResponseGetBreakInTargetListParams params;
        else if (paramsType == ParamsType::AllowBreakInTarget)
            ResponseAllowBreakInTargetParams params;
        else if (paramsType == ParamsType::GetVisitorList)
            ResponseGetVisitorListParams params;
        else if (paramsType == ParamsType::Visit)
            ResponseVisitParams params;
        else if (paramsType == ParamsType::UpdatePlayerStatus)
            ResponseUpdatePlayerStatusParams params;
        else if (paramsType == ParamsType::GRGetPlayerEquipments)
            ResponseGRGetPlayerEquipmentsParams params;
        else if (paramsType == ParamsType::GRUploadPlayerEquipments)
            ResponseGRUploadPlayerEquipmentsParams params;
    }
};
