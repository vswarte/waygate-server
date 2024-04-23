struct RequestCreateBloodMessageParams {
    OnlineArea area;
    u32 characterId;
    Vector<char> data;
    u32 unk;
    GroupPasswords groupPasswords;
};

struct ResponseCreateBloodMessageParams {
    u32 messageId;
    u32 sessionId;
};
