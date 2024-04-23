struct GhostData {
    u32 unk1;
    u32 unk2;
    u32 unk3;
    u32 unk4;
    u32 unk5;
    u32 unk6;
    float x;
    float y;
    float z;
    u32 unk10;
    u32 unk11;
    u32 unk12;
    u32 unk13;
    u32 compressedSize;
    u32 decompressedSize;
    
    char compressedReplayData[compressedSize];
};

GhostData ghostData @ 0x0;
