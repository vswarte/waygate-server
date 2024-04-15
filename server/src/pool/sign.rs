use super::pool::PoolEntryMatcher;
use crate::pool::matching::weapon;

use super::matching::area::MatchingArea;

#[derive(Clone, Debug)]
pub struct SignPoolEntry {
    pub character_level: u16,
    pub weapon_level: u16,
    pub area: MatchingArea,
    pub password: String,
    pub group_passwords: Vec<String>,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct SignPoolQuery {
    pub character_level: u16,
    pub weapon_level: u16,
    pub areas: Vec<MatchingArea>,
    pub password: String,
}

pub struct SignHostMatcher;

impl SignHostMatcher {
    fn check_character_level(host: u16, finger: u16) -> bool {
        let lower = host - (host / 10);
        let upper = host + (host / 10) + 10;

        finger >= lower && finger <= upper
    }

    fn check_weapon_level(host: u16, finger: u16) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&finger)
        } else {
            false
        }
    }
}

impl PoolEntryMatcher<SignPoolEntry, SignPoolQuery> for SignHostMatcher {
    fn matches(entry: &SignPoolEntry, query: &SignPoolQuery) -> bool {
        // TODO: we can probably index entries by area instead
        if !query.areas.contains(&entry.area) {
            false
        } else if !entry.password.is_empty() || !query.password.is_empty() {
            entry.password == query.password
        } else {
            Self::check_character_level(query.character_level, entry.character_level)
                && Self::check_weapon_level(query.weapon_level, entry.weapon_level)
        }
    }
}

#[cfg(test)]
mod test {
    use super::PoolEntryMatcher;
    use crate::pool::matching::area::MatchingArea;
    use crate::pool::sign::{SignHostMatcher, SignPoolEntry, SignPoolQuery};

    #[test]
    fn test_character_level() {
        assert!(SignHostMatcher::check_character_level(1, 300) == false);
        assert!(SignHostMatcher::check_character_level(28, 31) == true);
    }

    #[test]
    fn test_weapon_level() {
        assert!(SignHostMatcher::check_weapon_level(0, 0) == true);
        assert!(SignHostMatcher::check_weapon_level(0, 2) == true);
        assert!(SignHostMatcher::check_weapon_level(0, 3) == false);
        assert!(SignHostMatcher::check_weapon_level(12, 14) == true);
        assert!(SignHostMatcher::check_weapon_level(12, 8) == true);
        assert!(SignHostMatcher::check_weapon_level(12, 25) == false);
    }

    #[test]
    fn level_1_characters_match() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::new(1, 1)],
            password: String::default(),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == true);
    }

    #[test]
    fn doesnt_match_differing_levels() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 100,
            weapon_level: 1,
            areas: vec![MatchingArea::new(1, 1)],
            password: String::default(),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == false);
    }

    #[test]
    fn password_matches_regardless() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::from("test"),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 713,
            weapon_level: 1,
            areas: vec![MatchingArea::new(1, 1)],
            password: String::from("test"),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == true);
    }

    #[test]
    fn doesnt_match_on_differing_passwords() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::from("123"),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::new(1, 1)],
            password: String::from("456"),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == false);
    }

    #[test]
    fn doesnt_match_when_password_isnt_set_on_finger() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::new(1, 1)],
            password: String::from("456"),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == false);
    }

    #[test]
    fn search_areas_work() {
        let finger = SignPoolEntry {
            character_level: 1,
            weapon_level: 1,
            area: MatchingArea::new(1, 1),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
        };

        let host = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::new(2, 2)],
            password: String::default(),
        };

        assert!(SignHostMatcher::matches(&finger, &host) == false);
    }
}
