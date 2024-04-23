struct RequestGetSignListParams {
    Vector<ObjectIdentifier> knownSigns;
    Vector<OnlineArea> searchAreas;
    // TODO
    MatchingParameters matchingParameters;
};

struct ResponseGetSignListParamsEntry {
    u32 playerId;
    ObjectIdentifier identifier;
    OnlineArea area;
    Vector<char> data;
    String steamId;
    String unkString;
    GroupPasswords groupPasswords;
};

struct ResponseGetSignListParams {
    Vector<ObjectIdentifier> knownSigns;
    Vector<ResponseGetSignListParamsEntry> entries;
};
