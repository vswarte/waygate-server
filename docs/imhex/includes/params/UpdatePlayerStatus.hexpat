struct CharacterDataStats {
    u32 hp;
    u32 maxHp;
    u32 baseMaxHp;
    u32 fp;
    u32 maxFp;
    u32 baseMaxFp;
    u32 stamina;
    u32 maxStamina;
    u32 baseMaxStamina;
};

struct CharacterDataAttributes {
    u32 vigor;
    u32 mind;
    u32 endurance;
    u32 vitality;
    u32 strength;
    u32 dexterity;
    u32 intelligence;
    u32 faith;
    u32 arcane;
};

struct CharacterDataAttack {
    u32 rightArmament1;
    u32 rightArmament2;
    u32 rightArmament3;
    u32 leftArmament1;
    u32 leftArmament2;
    u32 leftArmament3;
};

struct CharacterDataDefense {
    u32 physical;
    u32 strike;
    u32 slash;
    u32 pierce;
    u32 magic;
    u32 fire;
    u32 lightning;
    u32 holy;
};

struct CharacterDataDamageNegation {
    u32 physical;
    u32 strike;
    u32 slash;
    u32 pierce;
    u32 magic;
    u32 fire;
    u32 lightning;
    u32 holy;
};

struct CharacterDataResistance {
    u32 immunity;
    u32 robustness;
    u32 focus;
    u32 vitality;
};

struct SiteOfGrace {
    u32 siteOfGrace;
    u8 discovered;
};

struct EquippedWeapon {
    u32 weapon;
    u32 ashOfWar;
};

struct EquippedProtector {
    u32 protector;
    u32 unk;
};

struct CharacterEquipment {
    Vector<EquippedWeapon> weaponsLeftHand;
    Vector<EquippedWeapon> weaponsRightHand;

    EquippedProtector head;
    EquippedProtector chest;
    EquippedProtector arms;
    EquippedProtector legs;

    Vector<u32> talismans;
    Vector<u32> quickSlots;
    Vector<u32> menuSlots;
    Vector<u32> arrowSlots;
    Vector<u32> boltSlots;
    Vector<u32> spellSlots;
};

struct CharacterData {
    u32 level;
    String characterName;
    u8 onlineActivity;
    u32 runesOwned;
    u32 totalRunesOwned;
    u8 unk4;
    u32 unk5;
    CharacterDataStats stats;
    CharacterDataAttributes attributes;
    float equipLoad;
    float maxEquipLoad;
    float poise;
    u32 discovery;
    CharacterDataAttack attackPower;
    CharacterDataDefense defense;
    CharacterDataDamageNegation damageNegation;
    CharacterDataResistance resistance;
    u32 unk6;
    u32 unk7;
    u32 unk8;
    u32 unk9;
    u32 unk10;
    u32 unk11;
    u32 unk12;
    u32 unk13;
    Vector<u32> unk14;
    Vector<u32> visitedAreas;
    u32 unk18[8];
    u32 maxReinforceLevel;
    u32 unk20;
    u8 unk21[0x27];
    String password;
    Vector<String> groupPasswords;
    u16 unk22;
    u8 unk24;
    u8 unk25;
    Vector<SiteOfGrace> graces;
    u8 unk26[0x18];
    CharacterEquipment equipment;
    u32 unk27;
    u32 unk28;
    u32 unk29;
    u32 unk30;
    u32 unk31;
    s32 unk32;
    u32 unk33;
};

struct RequestUpdatePlayerStatusParams {
    u32 unk1;
    u32 mapId;
    u32 unk2; // ordinarily 0x0, changes to 0x2 in arena?
    u32 deathCount;
    u32 totalSummonCount;
    u32 coopSuccessCount;

    u32 invadersKilledCount;
    u32 hostsKilledCount;

    u32 gameVersion;
    u32 unk7;
    u8 unk8;

    CharacterData character;
};

struct ResponseUpdatePlayerStatusParams {
};
