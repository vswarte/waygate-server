struct RequestGRGetPlayerEquipmentsParams {
    u32 poolType;
    u32 unk2;
    u32 unk3;
};

struct ResponseGRGetPlayerEquipmentsParamsEntry {
    u32 entryId;
    Vector<char> data;
};

struct ResponseGRGetPlayerEquipmentsParams {
    Vector<ResponseGRGetPlayerEquipmentsParamsEntry> entries;
};
