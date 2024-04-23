#include <std/string.pat>

struct ObjectIdentifier {
    u32 objectId;
    u32 unkId; // Could be a session ID?
} [[format("format_object_identifier")]];

// Types to indicate the request/response itself is empty and the status code is all that is required
struct EmptyRequest {};
struct ResponseOk {};

struct SessionData {
    ObjectIdentifier sessionIdentifier;
    u64 validFrom;
    u64 validUntil;
    String sessionCookie;
};

struct MapRelativeLocation {
    u32 mapId;
    float x;
    float y;
    float z;
} [[format("format_map_relative_location")]];

fn format_map_relative_location(ref MapRelativeLocation input) { 
    return std::string::to_string(input.mapId) + ": " +
        std::string::to_string(input.x) + "," +
        std::string::to_string(input.y) + "," +
        std::string::to_string(input.z);
};

enum ItemCategory : u32 {
    Goods = 0x4,
};

using GroupPasswords = Vector<String>;

struct OnlineArea {
    u32 area;
    u32 playRegion;
} [[format("format_online_area")]];

fn format_online_area(ref OnlineArea input) { 
    return std::string::to_string(input.mapId) + ":" + std::string::to_string(input.onlineAreaId);
};

fn format_object_identifier(ref ObjectIdentifier input) { 
    return std::string::to_string(input.objectId) + ":" + std::string::to_string(input.unkId);
};

bitfield RegionFlags {
    unk0 : 1;
    unk1 : 1;
    unk2 : 1;
    unk3 : 1;
    unk4 : 1;
    unk5 : 1;
    unk6 : 1;
    unk7 : 1;
    unk8 : 1;
    unk9 : 1;
    unk10 : 1;
    unk11 : 1;
    unk12 : 1;
    unk13 : 1;
    unk14 : 1;
    unk15 : 1;
    unk16 : 1;
    unk17 : 1;
    unk18 : 1;
    unk19 : 1;
    unk20 : 1;
    unk21 : 1;
    unk22 : 1;
    unk23 : 1;
    unk24 : 1;
    unk25 : 1;
    unk26 : 1;
    unk27 : 1;
    unk28 : 1;
    unk29 : 1;
    unk30 : 1;
    unk31 : 1;
};

struct MatchingParameters {
    u32 gameVersion;
    u32 unk1;
    RegionFlags regionFlags;
    u16 unk2;
    u16 soulLevel;
    u32 unk3;
    u32 unk4;
    u16 clearCount;
    String password;
    u32 unk5; // Could be covenant?
    u16 maxReinforce;
    u16 unk6;
};
