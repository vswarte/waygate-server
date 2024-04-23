struct RequestCreateSignParams {
    OnlineArea area;
    MatchingParameters MatchingParameters;
    u32 unk0;
    Vector<char> data;
    GroupPasswords groupPasswords;
};

struct ResponseCreateSignParams {
    ObjectIdentifier identifier;
};
