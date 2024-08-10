use crate::{MatchResult, PoolQuery};
use crate::matching::weapon;

use waygate_message::{RequestRegisterQuickMatchParams, RequestSearchQuickMatchParams, ResponseSearchQuickMatchParamsEntry};

#[derive(Debug, Default)]
pub struct QuickmatchPoolEntry {
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub arena_id: i32,
    pub password: String,
    pub settings: i32,
}

impl QuickmatchPoolEntry {
    pub fn from_request(val: RequestRegisterQuickMatchParams, external_id: String) -> Self {
        QuickmatchPoolEntry {
            external_id,
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
            arena_id: val.arena_id,
            password: val.matching_parameters.password.clone(),
            settings: val.quickmatch_settings,
        }
    }
}

impl Into<ResponseSearchQuickMatchParamsEntry> for &MatchResult<QuickmatchPoolEntry> {
    fn into(self) -> ResponseSearchQuickMatchParamsEntry {
        ResponseSearchQuickMatchParamsEntry {
            host_player_id: self.0.1,
            host_steam_id: self.1.external_id.clone(),
            arena_id: self.1.arena_id,
        }
    }
}

#[derive(Debug)]
pub struct QuickmatchPoolQuery {
    pub arena_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub password: String,
    pub settings: i32,
}

impl QuickmatchPoolQuery {
    fn check_character_level(host: u32, joiner: u32) -> bool {
        let lower = host - (host / 10);
        let upper = host + (host / 10) + 10;

        joiner >= lower && joiner <= upper
    }

    fn check_weapon_level(host: u32, joiner: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&joiner)
        } else {
            false
        }
    }
}

impl PoolQuery<QuickmatchPoolEntry> for QuickmatchPoolQuery {
    fn matches(&self, entry: &QuickmatchPoolEntry) -> bool {
        // TODO: password team matching
        if self.arena_id != entry.arena_id || self.settings != entry.settings {
            return false;
        }

        if !entry.password.is_empty() || !self.password.is_empty() {
            return entry.password.eq(&self.password);
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }
}

impl From<RequestSearchQuickMatchParams> for QuickmatchPoolQuery {
    fn from(value: RequestSearchQuickMatchParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            password: value.matching_parameters.password.clone(),
            arena_id: value.arena_id,
            settings: value.quickmatch_settings,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PoolQuery;
    use crate::quickmatch::{QuickmatchPoolEntry, QuickmatchPoolQuery};

    #[test]
    fn test_character_level() {
        assert!(!QuickmatchPoolQuery::check_character_level(1, 300));
        assert!(QuickmatchPoolQuery::check_character_level(28, 31));
    }

    #[test]
    fn test_weapon_level() {
        assert!(QuickmatchPoolQuery::check_weapon_level(0, 0));
        assert!(QuickmatchPoolQuery::check_weapon_level(0, 2));
        assert!(!QuickmatchPoolQuery::check_weapon_level(0, 4));
        assert!(QuickmatchPoolQuery::check_weapon_level(12, 14));
        assert!(QuickmatchPoolQuery::check_weapon_level(12, 8));
        assert!(!QuickmatchPoolQuery::check_weapon_level(12, 25));
    }

    #[test]
    fn level_1_characters_match() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_differing_levels() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 100,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn password_matches_regardless() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::from("test"),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 713,
            weapon_level: 1,
            password: String::from("test"),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_on_differing_passwords() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::from("123"),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::from("456"),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_when_password_isnt_set_on_joiner() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::from("456"),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_mismatching_arena_ids() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x1,
            settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_mismatching_settings() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x1,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn test_clayamore_group() {
        let host = QuickmatchPoolEntry {
            external_id: String::new(),
            character_level: 130,
            weapon_level: 25,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        let joiner = QuickmatchPoolQuery {
            character_level: 137,
            weapon_level: 25,
            password: String::default(),
            arena_id: 0x0,
            settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }
}
