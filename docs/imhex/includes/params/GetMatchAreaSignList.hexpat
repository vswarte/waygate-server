struct RequestGetMatchAreaSignListParams {
    u32 unk1;
    u32 unk2;
    u32 matchAreaId;
    u32 unk4;
    u32 unk5;
    u32 unk6;
    u32 unk7;
    u32 unk8;
    u32 unk9;
    u32 unk10;
    u32 unk11;
    u32 unk12;
};

struct ResponseGetMatchAreaSignListParamsEntry {
    u32 playerId;
    ObjectIdentifier identifier;
    u32 matchAreaId;
    u32 unk2;
    u32 unk3;
    Vector<char> data;
    String steamId; 
    String unkString; 
    GroupPasswords groupPasswords;
};

struct ResponseGetMatchAreaSignListParams {
    u32 unk0;
    Vector<ResponseGetMatchAreaSignListParamsEntry> entries;
};
