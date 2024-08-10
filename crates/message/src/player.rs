use serde::{Serialize, Deserialize};

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdatePlayerStatusParams {
    pub unk1: u32,
    pub map_id: u32,
    pub unk2: u32,
    pub death_count: u32,
    pub total_summon_count: u32,
    pub coop_success_count: u32,
    pub invaders_killed_count: u32,
    pub hosts_killed_count: u32,
    pub game_version: u32,
    pub unk7: u8,
    pub unk8: u32,
    pub character: CharacterData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterData {
    pub level: u32,
    pub character_name: String,

    pub online_activity: u8,

    pub runes_owned: u32,
    pub total_runes_owned: u32,
    pub unk4: u8,
    pub unk5: u32,

    pub stats: CharacterDataStats,
    pub attributes: CharacterDataAttributes,

    pub equip_load: f32,
    pub max_equip_load: f32,
    pub poise: f32,
    pub discovery: u32,

    pub attack_power: CharacterDataAttack,
    pub defense: CharacterDataDefense,
    pub damage_negation: CharacterDataDamageNegation,
    pub resistance: CharacterDataResistance,

    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: Vec<u32>,
    pub visited_areas: Vec<u32>,
    pub unk15: [u32; 8],
    pub max_reinforce_level: u32,
    pub unk20: u32,

    // This is cursed. Apparently serde only takes u8 arrays of size n % 8
    //pub unk21: [u8; 0x27],
    pub unk21: [u8; 0x20],
    pub wtf1: u8,
    pub wtf2: u8,
    pub wtf3: u8,
    pub wtf4: u8,
    pub wtf5: u8,
    pub wtf6: u8,
    pub wtf7: u8,

    pub password: String,
    pub group_passwords: Vec<String>,
    pub unk22: u16,
    pub unk24: u8,
    pub unk25: u8,
    pub sites_of_grace: Vec<SiteOfGrace>,
    pub unk26: [u8; 0x18],
    pub equipment: CharacterEquipment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataStats {
    pub hp: u32,
    pub max_hp: u32,
    pub base_max_hp: u32,
    pub fp: u32,
    pub max_fp: u32,
    pub base_max_fp: u32,
    pub stamina: u32,
    pub max_stamina: u32,
    pub base_max_stamina: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataAttributes {
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub vitality: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataAttack {
    pub right_armament_primary: u32,
    pub right_armament_secondary: u32,
    pub right_armament_tertiary: u32,
    pub left_armament_primary: u32,
    pub left_armament_secondary: u32,
    pub left_armament_tertiary: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataDefense {
    pub physical: u32,
    pub strike: u32,
    pub slash: u32,
    pub pierce: u32,
    pub magic: u32,
    pub fire: u32,
    pub lightning: u32,
    pub holy: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataDamageNegation {
    pub physical: u32,
    pub strike: u32,
    pub slash: u32,
    pub pierce: u32,
    pub magic: u32,
    pub fire: u32,
    pub lightning: u32,
    pub holy: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterDataResistance {
    pub immunity: u32,
    pub robustness: u32,
    pub focus: u32,
    pub vitality: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteOfGrace {
    pub site_of_grace: u32,
    pub discovered: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquippedWeapon {
    pub weapon: u32,
    pub ash_of_war: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquippedProtector {
    pub protector: u32,
    pub unk: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterEquipment {
    pub weapons_left_hand: Vec<EquippedWeapon>,
    pub weapons_right_hand: Vec<EquippedWeapon>,

    pub head: EquippedProtector,
    pub chest: EquippedProtector,
    pub arms: EquippedProtector,
    pub legs: EquippedProtector,

    pub talismans: Vec<u32>,
    pub quickslots: Vec<u32>,
    pub menuslots: Vec<u32>,
    pub arrows: Vec<u32>,
    pub bolts: Vec<u32>,
    pub spells: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUseItemLogParams {
    pub used_items: Vec<RequestUseItemLogParamsEntry>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUseItemLogParamsEntry {
    pub item_id: u32,
    pub times_used: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestGetItemLogParams {
    pub acquired_items: Vec<RequestGetItemLogParamsEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestGetItemLogParamsEntry {
    pub location: Location,
    pub item_category: u32,
    pub item_id: u32,
    pub quantity: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestKillEnemyLogParams {
    pub killed_enemies: Vec<RequestKillEnemyLogParamsEntry>,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestKillEnemyLogParamsEntry {
    pub npc_param: u32,
    pub killed_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestJoinMultiplayParams {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}
