struct RequestGetDeadingGhostParams {
    OnlineArea area;
    ObjectIdentifier identifier;
};

struct ResponseGetDeadingGhostParams {
    u32 unk1;
    u32 unk2;
    ObjectIdentifier identifier;
    Vector<char> data;
};
