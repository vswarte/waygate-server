struct ResponseCreateSessionParams {
    u32 playerId;
    String steamId;
    String ipAddress;
    SessionData sessionData;
    String unkString;
};

struct RequestCreateSessionParams {
    u32 unk;
    u32 unk1;
    u32 unk2;
    u32 gameVersion;
    u32 unk3;
    u32 unk4;
    Vector<char> steamEncryptedAppTicket;
    u32 unk5;
    u32 unk6;
    u32 unk7;
    u32 unk8;
    u32 unk9;
    u32 unk10;
    u32 unk11;
    u32 unk12;
    u16 unk13;
};
