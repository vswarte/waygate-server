struct RequestCreateBloodstainParams {
    OnlineArea area;
    Vector<char> metaData;
    Vector<char> replayData;
    GroupPasswords groupPasswords;
};

struct ResponseCreateBloodstainParams {
    u32 unk1;
    u32 unk2;
};
