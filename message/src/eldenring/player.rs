use serde::{Deserialize, Serialize};
use wire::ShiftJisString;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestUpdatePlayerStatusParams {
    /// NetPlayerWatcher->0x10, always 0 for PC builds.
    pub unk1: u32,
    pub play_region: u32,
    pub game_clear_count: u32,
    pub death_count: u32,
    pub total_summon_count: u32,
    pub coop_success_count: u32,
    pub invaders_killed_count: u32,
    pub hosts_killed_count: u32,
    pub character: CharacterData,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum NetStatus {
    Offline = 0,
    Host = 1,
    Client = 2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiplayerData {
    /// CSNetMan->0xC
    pub unk1: u32,
    /// The platform the player is on, always 0 for PC builds
    pub platform: u8,
    /// Amount of players in the session (including the NPC)
    pub player_count: u32,
    pub unk2: u8,
    /// Whether the player can be invaded by other players
    pub is_invadeable: bool,
    /// Whether the player has reached the maximum rune memory
    pub reached_max_rune_memory: bool,
    /// Whether the player can be summoned by other players as a hunter
    pub can_be_hunter: bool,
    /// Whether the player can be invaded by hunters (e.g. Blades of the Darkmoon)
    pub can_be_invaded_by_hunters: bool,
    pub unk3: u8,
    /// Contains the team types of the players in the world (excluding the Host)
    pub chr_team_types: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterStatistics {
    /// Play time in seconds, capped at 3599999 (999 hours, 59 minutes, 59 seconds)
    pub play_time: u32,
    pub sites_of_grace: Vec<SiteOfGrace>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterData {
    pub regulation_version: u32,
    pub sell_region: SellRegion,
    pub cross_region_matchmaking_disabled: bool,
    pub level: u32,
    pub character_name: ShiftJisString,
    pub furled_finger_enabled: bool,
    pub runes_owned: u32,
    pub total_runes_owned: u32,
    pub archetype: u32,
    pub gender: u8,
    pub stats: CharacterDataStats,
    pub attributes: CharacterDataAttributes,
    pub equip_load: f32,
    pub max_equip_load: f32,
    pub poise: f32,
    pub discovery: u32,
    pub attack_power: CharacterDataAttack,
    pub defense: CharacterDataDefense,
    /// Only read in message serialization, never written to
    pub unused_resistances: [f32; 8],
    /// Proc status gauges, 0 means status is active
    pub gauges: CharacterDataGauges,
    /// Only read in message serialization, never written to
    pub unused_gauge_timers: [u32; 4],
    pub hp_flasks: u32,
    pub fp_flasks: u32,
    pub unk1: u32,
    pub unk2: u32,
    /// list of DLCs owned by the player where each id is 1 or 0
    /// e.g. [1, 1, 0] means the player owns preorder bonus gesture, SotE but not the SotE preorder bonus
    pub owned_dlcs: Vec<u32>,
    pub visited_areas: Vec<u32>,
    pub scadutree_blessing: u32,
    pub reversed_spirit_ash_blessing: u32,
    pub has_beat_dlc: u32,
    pub unk3: [u32; 4],
    pub solo_break_in_point: u32,
    pub max_reinforce_level: u32,
    pub unk4: u8,
    pub unk5: u32,
    pub unk6: [u8; 11],
    pub multiplayer_data: MultiplayerData,
    pub unk7: Vec<u32>,
    pub password: ShiftJisString,
    /// For some reason, group passwords are encoded as utf8 strings instead of ShiftJis
    pub group_passwords: Vec<String>,
    pub statistics: CharacterStatistics,
    pub equipment: CharacterEquipment,
    pub unk8: u32,
    pub net_status: NetStatus,
    /// Datetime in format "YYYYMMDDHHmmss000000000IsUTC"
    pub unk9: ShiftJisString,
    pub unk10: i32,
    pub unk11: u32,
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
    pub right_armament_primary: i32,
    pub right_armament_secondary: i32,
    pub right_armament_tertiary: i32,
    pub left_armament_primary: i32,
    pub left_armament_secondary: i32,
    pub left_armament_tertiary: i32,
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
pub struct CharacterDataGauges {
    pub bleed: u32,
    pub poison: u32,
    pub frost: u32,
    pub death_blight: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SiteOfGrace {
    pub site_of_grace: u32,
    pub discovered: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquippedWeapon {
    pub weapon: u32,
    pub gem: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EquippedProtector {
    pub protector: u32,
    pub durability: u32,
}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ChrAsmSlot {
    WeaponLeft1 = 0,
    WeaponRight1 = 1,
    WeaponLeft2 = 2,
    WeaponRight2 = 3,
    WeaponLeft3 = 4,
    WeaponRight3 = 5,
    Arrow1 = 6,
    Bolt1 = 7,
    Arrow2 = 8,
    Bolt2 = 9,
    Arrow3 = 10,
    Bolt3 = 11,
    ProtectorHead = 12,
    ProtectorChest = 13,
    ProtectorHands = 14,
    ProtectorLegs = 15,
    Unused16 = 16,
    Accessory1 = 17,
    Accessory2 = 18,
    Accessory3 = 19,
    Accessory4 = 20,
    AccessoryCovenant = 21,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterEquipment {
    pub weapons_left_hand: Vec<EquippedWeapon>,
    pub weapons_right_hand: Vec<EquippedWeapon>,
    pub head: EquippedProtector,
    pub chest: EquippedProtector,
    pub arms: EquippedProtector,
    pub legs: EquippedProtector,
    pub accessories: Vec<i32>,
    pub quickslots: Vec<i32>,
    pub pouchslots: Vec<i32>,
    pub arrows: Vec<i32>,
    pub bolts: Vec<i32>,
    pub spells: Vec<i32>,
    pub right_hand_slot: ChrAsmSlot,
    pub left_hand_slot: ChrAsmSlot,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseUpdatePlayerStatusParams {}

#[repr(u32)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum JoinMultiplayState {
    Host = 0,
    Client = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestJoinMultiplayParams {
    pub unk1: u32,
    pub play_region_param_id: u32,
    pub level: u32,
    pub unk4: u32,
    pub state: JoinMultiplayState,
    pub max_reinforce_level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseJoinMultiplayParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestLeaveMultiplayParams {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseLeaveMultiplayParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateLoginPlayerCharacterParams {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUpdateLoginPlayerCharacterParams {
    pub character_id: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

#[cfg(test)]
mod test {
    use crate::eldenring::NetStatus;

    use super::RequestUpdatePlayerStatusParams;
    use wire::deserialize;

    #[test]
    fn deserialize_update_player_status() {
        let deserialized: RequestUpdatePlayerStatusParams = deserialize(include_bytes!(
            "../../test/data/RequestUpdatePlayerStatus.bin"
        ))
        .unwrap();

        assert_eq!(deserialized.character.attributes.vigor, 99);
        assert_eq!(deserialized.character.net_status, NetStatus::Offline);
        assert_eq!(deserialized.character.multiplayer_data.platform, 0);
        assert_eq!(deserialized.character.multiplayer_data.player_count, 1);
        assert!(deserialized.character.multiplayer_data.can_be_hunter);
        assert!(
            !deserialized
                .character
                .multiplayer_data
                .can_be_invaded_by_hunters
        );
        assert!(deserialized.character.multiplayer_data.is_invadeable);
    }
}
