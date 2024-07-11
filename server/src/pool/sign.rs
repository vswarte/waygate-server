use super::{matching::area::MatchingArea, PoolQuery};
use crate::pool::matching::weapon;

#[derive(Debug, Default)]
pub struct SignPoolEntry {
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub area: MatchingArea,
    pub password: String,
    pub group_passwords: Vec<String>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SignPoolQuery {
    pub character_level: u32,
    pub weapon_level: u32,
    pub areas: Vec<MatchingArea>,
    pub password: String,
}

impl SignPoolQuery {
    fn check_character_level(host: u32, finger: u32) -> bool {
        let lower = host - (host / 10);
        let upper = host + (host / 10) + 10;

        finger >= lower && finger <= upper
    }

    fn check_weapon_level(host: u32, finger: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&finger)
        } else {
            false
        }
    }
}

impl PoolQuery<SignPoolEntry> for SignPoolQuery {
    fn matches(&self, entry: &SignPoolEntry) -> bool {
        if !self.areas.iter().any(|c| c.matches(&entry.area)) {
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
    use crate::pool::matching::area::MatchingArea;
    use crate::pool::PoolQuery;
    use crate::pool::sign::{SignPoolEntry, SignPoolQuery};

    #[test]
    fn test_character_level() {
        assert!(!SignPoolQuery::check_character_level(1, 300));
        assert!(SignPoolQuery::check_character_level(28, 31));
    }

    #[test]
    fn test_weapon_level() {
        assert!(SignPoolQuery::check_weapon_level(0, 0));
        assert!(SignPoolQuery::check_weapon_level(0, 2));
        assert!(!SignPoolQuery::check_weapon_level(0, 4));
        assert!(SignPoolQuery::check_weapon_level(12, 14));
        assert!(SignPoolQuery::check_weapon_level(12, 8));
        assert!(!SignPoolQuery::check_weapon_level(12, 25));
    }

    #[test]
    fn level_1_characters_match() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::default(),
        };

        assert!(finger.matches(&host));
    }

    #[test]
    fn doesnt_match_differing_levels() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 100,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::default(),
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn password_matches_regardless() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::from("test"),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 713,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::from("test"),
        };

        assert!(finger.matches(&host));
    }

    #[test]
    fn doesnt_match_on_differing_passwords() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::from("123"),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::from("456"),
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn doesnt_match_when_password_isnt_set_on_host() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::from("456"),
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn doesnt_match_across_search_areas() {
        let host = SignPoolEntry {
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::PlayRegion { area: 1, play_region: 1},
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let finger = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 1, play_region: 1}],
            password: String::default(),
        };

        assert!(!finger.matches(&host));
    }
}
