use super::PoolQuery;
use crate::pool::matching::weapon;

#[derive(Debug, Default)]
pub struct QuickmatchPoolEntry {
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub arena_id: i32,
    pub password: String,
    pub settings: i32,
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

#[cfg(test)]
mod test {
    use crate::pool::PoolQuery;
    use crate::pool::quickmatch::{QuickmatchPoolEntry, QuickmatchPoolQuery};

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
