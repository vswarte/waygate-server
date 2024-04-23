enum JoinType : u32 {
    InvaderJoining = 0x2,
    JoiningAsInvader = 0x3,

    PlayerJoining = 0x5,
    JoiningPlayer = 0x6,

    JoiningAsBlue = 0x9,

    PlayerJoiningQuickMatch = 0xB,
    JoiningQuickMatch = 0xC,
};

struct PushInvaderJoiningParams {
    u32 invaderPlayerId;
    String invaderSteamId;
    u32 unk1;
    u32 unk2;
    u32 matchedOnlineAreaId;
};

struct PushJoiningAsInvaderParams {
    u32 hostPlayerId;
    Vector<char> joinData;
    u32 unk1;
};

struct PushPlayerJoiningParams {
    u32 playerId;
    ObjectIdentifier signIdentifier;
};

struct PushJoiningPlayerParams {
    u32 hostPlayerId;
    String summonedPlayerSteamId;
    u32 summonedPlayerId;
    ObjectIdentifier signIdentifier;
    Vector<char> joinData;
};

struct PushJoiningAsBlueParams {
    u32 hostPlayerId;
    String hostPlayerSteamId;
    Vector<char> joinData;
    u32 unk1;
    u32 unk2;
    u32 mapId;
};

struct PushJoiningQuickMatchParams {
    u32 quickmatchSettings;
    u32 hostPlayerId;
    String hostSteamId;
    Vector<char> joinData;
};

struct PushPlayerJoiningQuickMatchParams {
    u32 quickmatchSettings;
    u32 joiningPlayerId;
    String joiningPlayerSteamId;
    u32 unk3;
    u32 arenaId;
    u8 unk4;
    String password;
};

struct PushJoinParams {
    ObjectIdentifier unkIdentifier;
    JoinType joinType;

    if (joinType == JoinType::InvaderJoining)
        PushInvaderJoiningParams params;
    else if (joinType == JoinType::JoiningAsInvader)
        PushJoiningAsInvaderParams params;
    else if (joinType == JoinType::PlayerJoining)
        PushPlayerJoiningParams params;
    else if (joinType == JoinType::JoiningPlayer)
        PushJoiningPlayerParams params;
    else if (joinType == JoinType::JoiningAsBlue)
        PushJoiningAsBlueParams params;
    else if (joinType == JoinType::PlayerJoiningQuickMatch)
        PushPlayerJoiningQuickMatchParams params;
    else if (joinType == JoinType::JoiningQuickMatch)
        PushJoiningQuickMatchParams params;
};

struct PushNotifyParamsSection1Variant1 {
    u32 unk1;
    u32 unk2;
};

struct PushNotifyParamsSection1Variant2 {
    u8 unk1;
    String matchedGroupPassword;
};

struct PushNotifyParamsSection1 {
    u32 type;
    if (type == 0x0)
        PushNotifyParamsSection1Variant1 body;
    else if (type == 0x1)
        PushNotifyParamsSection1Variant2 body;
};

struct PushNotifyParamsSection2Variant1 {
    String unkString;
    u64 unk1;
    u32 unk2;
    u32 unk3;
};

struct PushNotifyParamsSection2Variant2 {
    u32 unk1;
    String unkString;
    Vector<u8> unkBuffer;
};

struct PushNotifyParamsSection2 {
    u32 type;
    if (type == 0x0)
        PushNotifyParamsSection2Variant1 body;
    else if (type == 0x1)
        PushNotifyParamsSection2Variant2 body;
};

struct PushNotifyParams {
    ObjectIdentifier identifier;
    u64 timestamp;
    PushNotifyParamsSection1 section1;
    PushNotifyParamsSection2 section2;
};
