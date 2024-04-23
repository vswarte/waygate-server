struct RequestReentryBloodMessageParamsEntry {
    u32 unk0;
    u32 unk4;
};

struct RequestReentryBloodMessageParams {
    Vector<RequestReentryBloodMessageParamsEntry> entries;
    u32 unk;
};

struct ResponseReentryBloodMessageParams {
    Vector<ObjectIdentifier> entries;
};
