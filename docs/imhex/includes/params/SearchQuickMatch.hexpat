struct RequestSearchQuickMatchParams {
    u32 quickmatchSettings;
    u32 unk2;
    u32 arenaId;
    u32 unk4;
    MatchingParameters matchingParameters;
    u32 characterId;
};

struct ResponseSearchQuickMatchParamsEntry {
    u32 hostPlayerId;
    String hostSteamId;
    u32 arenaId;
};

struct ResponseSearchQuickMatchParams {
    Vector<ResponseSearchQuickMatchParamsEntry> matches;
    u16 unk1;
};
