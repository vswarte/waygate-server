struct RequestRestoreSessionParams {
    u32 gameVersion;
    u32 unk1;
    u32 unk2;
    Vector<char> steamEncryptedAppTicket;
    SessionData sessionData;
    u32 unk3;
};

struct ResponseRestoreSessionParams {
    SessionData sessionData;
};
