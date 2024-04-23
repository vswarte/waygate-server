#include <std/io.pat>

struct Map {
    u32 dword;
} [[format("read_map")]];

fn read_map(ref Map input) {
    u8 mapId      = (input.dword >> (3 * 8)) & 0xFF;
    u8 blockId    = (input.dword >> (2 * 8)) & 0xFF;
    u8 subBlockId = (input.dword >> (1 * 8)) & 0xFF;
    u8 ceremony   = (input.dword >> (0 * 8)) & 0xFF;

    return std::format(
        "map({}_{}_{}_{})",
        mapId, blockId, subBlockId, ceremony
    );
};


struct ChunkLocalPosAng {
    float x;
    float y;
    float z;
    float rotation;
};

struct ChunkPos {
    float x;
    float y;
    float z;
    float unk;
};

struct BloodstainAdvertisement {
    u32 unk1;
    u32 dataSize;
    u32 unk2;
    s32 unk3;
    u32 unk4;
    u32 unk5;
    u32 unk6;
    Map map;
    ChunkLocalPosAng coordinates;
    ChunkPos chunkPos;

    u32 unk16;
    u32 unk17;
    u32 unk18;
    u32 unk19;
    
    u32 playRegion;
    char16 playerName[16];

    u32 unk20;
    u32 unk21;
    u32 unk22;
};

BloodstainAdvertisement data @ 0x0;