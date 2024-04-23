struct RequestGetBreakInTargetListParams {
    u32 onlineAreaId;
    u32 unk2;
    MatchingParameters matchingParameters;
    u32 unk3;
};

struct ResponseGetBreakInTargetListParamsEntry {
    u32 targetPlayerId;
    String targetSteamId;
};

struct ResponseGetBreakInTargetListParams {
    u32 unk1;
    Vector<ResponseGetBreakInTargetListParamsEntry> entries;
};
