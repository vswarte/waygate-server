struct RequestGetVisitorListParams {
    u32 onlineAreaId;
    u32 unk1;
    MatchingParameters matchingParameters;
    u32 unk2;
    u8 unk3;
};

struct ResponseGetVisitorListParamsEntry {
    u32 playerId;
    String steamId;
    u32 unk1;
    u32 onlineAreaId;
    u32 unk3;
};

struct ResponseGetVisitorListParams {
    Vector<ResponseGetVisitorListParamsEntry> entries;
};
