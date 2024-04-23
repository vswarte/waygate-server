struct RequestGetBloodMessageListParams {
    Vector<OnlineArea> searchAreas;
    GroupPasswords groupPasswords;
};

struct ResponseGetBloodMessageListParamsEntry {
    u32 playerId;
    u32 characterId;
    ObjectIdentifier identifier;
    u32 ratingGood;
    u32 ratingBad;
    Vector<char> data;
    OnlineArea area;
    GroupPasswords groupPasswords;
};

struct ResponseGetBloodMessageListParams {
    Vector<ResponseGetBloodMessageListParamsEntry> entries;
};
