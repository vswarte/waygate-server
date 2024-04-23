#include <std/io.pat>

struct DP_REGULAR_GHOST_POSITION_AND_ORIENTATION {
    u32 dword1;
    u32 dword2;
} [[format("format_ghost_position_and_orientation")]];

fn format_ghost_position_and_orientation(ref DP_REGULAR_GHOST_POSITION_AND_ORIENTATION input) {
    /**
    ER 1.10: ChrPackingStructure::FUN_1404e5840
    
    dword1 = (param_1->dpGhost).dword1;
    dword2 = (param_1->dpGhost).dword2;
    x = (float)((int)(dword1 << 0xc) >> 0xc) * 0.02;
    z = (float)((dword2 << 7) >> 0xc) * 0.02;
    y = (float)((int)((dword2 << 0xf ^ dword1) & 0xf8000 ^ dword1) >> 0xf) * 0.04;
    angle = ((float)(ulonglong)((uint)(param_1->dpGhost).dword2 >> 0x19) - 64.0) * 0.015625 *
            3.141593;
    */

    float x = (u32(input.dword1 << 0xc) >> 0xc) * 0.02;
    float z = (u32(input.dword2 << 0x7) >> 0xc) * 0.02;
    float y = float(((input.dword2 << 0xf ^ input.dword1) & 0xf8000 ^ input.dword1) >> 0xf) * 0.04;
    float angle = (float(input.dword2 >> 0x19) - 64.0) * 0.015625 * 3.141593;

    return std::format("pos = ({:4f}, {:4f}, {:4f}), angle = {:4f}", x, y, z, angle);
};

struct DP_REGULAR_GHOST {
    u32 handednessState;
    float frameDuration;
    u32 mapId;
    
    DP_REGULAR_GHOST_POSITION_AND_ORIENTATION positionAndOrientation;
    
    u32 hpValues;
    s32 lastUsedMagic;
};

struct DP_OTHER_LOOK_DIRECTION {
    u32 dword1;
    u32 dword2;
} [[format("format_other_look_direction")]];

fn format_other_look_direction(ref DP_OTHER_LOOK_DIRECTION input) {
    /**
    ER 1.10: DP_REGULAR_OTHER_LOOK_DIRECTION::FUN_140c7f910
    
    positionDword1 = param_1->dword1;
    positionDword2 = param_1->dword2 >> 0x16;
    iVar1 = 1 << ((byte)positionDword1 & 7) + 3;
    fVar4 = (float)(ulonglong)(iVar1 * positionDword2) * 0.0009765625 + (float)(ulonglong)(iVar1 - 8);
    if (0x1fff < ((positionDword1 & 0xf) << 10 | positionDword2)) {
    fVar4 = (float)((uint)fVar4 ^ 0x80000000);
    }
    positionDword2 = positionDword1 >> 4 & 0x3fff;
    iVar1 = 1 << ((byte)(positionDword2 >> 10) & 7) + 3;
    fVar3 = (float)(ulonglong)(iVar1 * (positionDword1 >> 4 & 0x3ff)) * 0.0009765625 +
          (float)(ulonglong)(iVar1 - 8);
    if (0x1fff < positionDword2) {
    fVar3 = (float)((uint)fVar3 ^ 0x80000000);
    }
    iVar1 = 1 << ((byte)(positionDword1 >> 0x1c) & 7) + 3;
    fVar2 = (float)(ulonglong)(iVar1 * (positionDword1 >> 0x12 & 0x3ff)) * 0.0009765625 +
          (float)(ulonglong)(iVar1 - 8);
    if (0x1fff < positionDword1 >> 0x12) {
    fVar2 = (float)((uint)fVar2 ^ 0x80000000);
    }
    position->x = (float)(int)CONCAT44(fVar3,fVar2);
    position->y = (float)(int)(CONCAT44(fVar3,fVar2) >> 0x20);
    position->z = fVar4;
    angleDword2 = param_1->dword2;
    *(ulonglong *)angle =
       CONCAT44(((float)(ulonglong)(angleDword2 >> 7 & 0xff) * 0.00390625 - 0.5) * 6.283185,
                ((float)(ulonglong)(angleDword2 >> 0xf & 0x7f) * 0.0078125 - 0.5) * 6.283185);
    *(float *)((longlong)angle + 8) =
       ((float)(ulonglong)(angleDword2 & 0x7f) * 0.0078125 - 0.5) * 6.283185;
    */

    u32 dword1 = input.dword1;
    u32 dword2 = input.dword2;

    u32 a = 1 << (u8(dword1) & 7) + 3;
    float posX = float((dword2 >> 0x16) * a) * 0.0009765625 + float(a - 8);

    u32 b = dword1 >> 0x4 & 0x3fff;
    u32 c = 1 << (u8(b >> 0xA) & 7) + 3;
    float posY = ((dword1 >> 0x4 & 0x3ff) * c) * 0.0009765625 + float(c - 8);

    u32 d = 1 << ((dword1 >> 0x1c) & 7) + 3;
    float posZ = ((dword1 >> 0x12 & 0x3ff) * d) * 0.0009765625 + float(d - 8);

    float angleX = ((dword2 >> 0x7 & 0xff) * 0.00390625 - 0.5) * 6.283185;
    float angleY = ((dword2 >> 0xf & 0x7f) * 0.0078125 - 0.5) * 6.283185;
    float angleZ = ((dword2 & 0x7f) * 0.0078125 - 0.5) * 6.283185;

    return std::format(
        "pos = ({:4f}, {:4f}, {:4f}), angle = ({:4f}, {:4f}, {:4f})",
        posX, posY, posZ,
        angleX, angleY, angleZ
    );
};

struct DP_REGULAR_OTHER {
    s32 unk1;
    s32 unk2;
    DP_OTHER_LOOK_DIRECTION lookDirection;
    u8 unk3;
    u32 unk4;
    u16 unk5;
    u8 pad;
};

struct BehaviorStateSyncInfo0xDEntry {
    float mysteryFloat;
    u8 unk1;
    u16 count;
    // u32 unk2[count];
    u8 unkbytes[52];
};

struct BehaviorStateSyncInfo<auto size> {
    u8 type;
    
    if (type == 0xC) {
        u8 unk[5];
    } else if (type == 0xD) {
        //u32 unk1;
        u8 count;
        BehaviorStateSyncInfo0xDEntry entries[count];
    }
};

struct Keyframe {
    u32 size;
    u8 unk1;
    DP_REGULAR_GHOST dpGhost;
    u16 unk9;
    u16 unk10;
    u16 unk11;
    u16 unk12;
    DP_REGULAR_OTHER dpOther;
    s16 unk13;
    s16 unk14;
    s8 unk15;
    
    //BehaviorStateSyncInfo<size> behaviorStateSync;
    char data[size - 66];
} [[format("format_keyframe")]];

fn format_keyframe(ref Keyframe input) {
    // Return inner to format as char[] which gets displayed as a string
    return std::format(
        "duration = {:4f}",
        input.dpGhost.frameDuration
    );
};

struct PlayerKeyframeChannel {
    u32 frameCount;
    u32 unk1;
    Keyframe frames[frameCount];
};

struct MountKeyframeChannel {
    u32 frameCount;
    Keyframe frames[frameCount];
};

struct EquippedWeapon {
    s32 weaponId;
    s32 ashOfWarId;
    u32 unk;
};

struct Equipment {
    EquippedWeapon left1;    
    EquippedWeapon right1;
    EquippedWeapon left2;   
    EquippedWeapon right2; 
    EquippedWeapon left3;
    EquippedWeapon right3;
    s32 arrows1;
    s32 bolts1;
    s32 arrows2;
    s32 bolts2;
    
    s32 unk1;
    s32 unk2;
    
    s32 head;
    s32 chest;
    s32 gloves;
    s32 legs;
    
    s32 unk3;
    
    s32 talisman1;
    s32 talisman2;
    s32 talisman3;
    s32 talisman4;
    
    s32 unk4;
    
    s32 crystalTear1;
    s32 crystalTear2;
    
    s32 unk5;
};

struct FaceData {
    char magic[4];
    u32 unk1;
    
    // This is including the magic, unk1 and the size field itself
    u32 size;
    // char data[size - 12];
};

struct ReplayData {
    PlayerKeyframeChannel playerChannel;
    MountKeyframeChannel mountChannel;
    
    u32 unk2;
    u32 unk3;
    u32 unk4;
    u32 unk5;
    u32 unk6;
    u32 unk7;
    u32 unk8;
    u32 unk9;

    Equipment equipment;
       
    u32 unk11;
    u32 unk12;
    u32 unk13;
    u8 unk14;
    
    FaceData faceData;
};

ReplayData replayData @ 0x0;
