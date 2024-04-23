struct Attributes {
    u32 vigor;
    u32 mind;
    u32 endurance;
    u32 strength;
    u32 dexterity;
    u32 intelligence;
    u32 faith;
    u32 arcane;
};

struct EquippedWeapon {
    u32 unk1;
    u32 weapon;
    s32 ashOfWar;
};

struct Equipment {
    EquippedWeapon leftPrimary;
    EquippedWeapon rightPrimary;
    EquippedWeapon leftSecondary;
    EquippedWeapon rightSecondary;
    EquippedWeapon leftTertiary;
    EquippedWeapon rightTertiary;

    u32 unk1;

    s32 arrowsPrimary;
    s32 boltsPrimary;
    s32 arrowsSecondary;
    s32 boltsSecondary;

    s32 unk2;
    s32 unk3;

    u32 protectorHead;
    u32 protectorChest;
    u32 protectorArms;
    u32 protectorLegs;

    s32 unk4;
    s32 talismans[4];
};

struct PCInfo {
    u32 unk1;
    u32 soulLevel;

    Attributes attributes;
    Equipment equipment;
};

PCInfo pcInfo @ 0x0;
