use super::pool::PoolEntryMatcher;
use crate::pool::matching::weapon;

#[derive(Clone, Debug)]
pub struct BreakinPoolEntry {
    pub character_level: u16,
    pub weapon_level: u16,
    pub steam_id: String,
}

#[derive(Debug)]
pub struct BreakinPoolQuery {
    pub character_level: u16,
    pub weapon_level: u16,
}

// TODO: give a wet shit about invasion areas
pub struct BreakinHostMatcher;

impl BreakinHostMatcher {
    fn check_character_level(host: u16, invader: u16) -> bool {
        let lower = invader - (invader / 10);
        let upper = invader + (invader / 10) + 20;

        if invader >= 301 {
            host >= lower
        } else {
            host >= lower && host <= upper
        }
    }

    fn check_weapon_level(host: u16, invader: u16) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&invader)
        } else {
            false
        }
    }
}

impl PoolEntryMatcher<BreakinPoolEntry, BreakinPoolQuery> for BreakinHostMatcher {
    fn matches(entry: &BreakinPoolEntry, query: &BreakinPoolQuery) -> bool {
        Self::check_character_level(
            entry.character_level,
            query.character_level,
        ) && Self::check_weapon_level(
            entry.weapon_level,
            query.weapon_level,
        )
    }
}

#[cfg(test)]
mod test {
    use super::PoolEntryMatcher;
    use crate::pool::breakin::{BreakinPoolEntry, BreakinPoolQuery, BreakinHostMatcher};

    #[test]
    fn test_character_level() {
        assert!(BreakinHostMatcher::check_character_level(700, 400) == true);
        assert!(BreakinHostMatcher::check_character_level(1, 713) == false);
        assert!(BreakinHostMatcher::check_character_level(28, 31) == true);
        assert!(BreakinHostMatcher::check_character_level(54, 31) == true);
    }

    #[test]
    fn test_weapon_level() {
        assert!(BreakinHostMatcher::check_weapon_level(0, 0) == true);
        assert!(BreakinHostMatcher::check_weapon_level(0, 2) == true);
        assert!(BreakinHostMatcher::check_weapon_level(0, 3) == false);
        assert!(BreakinHostMatcher::check_weapon_level(12, 14) == true);
        assert!(BreakinHostMatcher::check_weapon_level(12, 8) == true);
        assert!(BreakinHostMatcher::check_weapon_level(12, 25) == false);
    }

    #[test]
    fn level_1_characters_match() {
        let host = BreakinPoolEntry {
            character_level: 1,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = BreakinPoolQuery {
            character_level: 1,
            weapon_level: 1,
        };

        assert!(BreakinHostMatcher::matches(&host, &invader) == true);
    }

    #[test]
    fn level_fall_off_applies() {
        let host = BreakinPoolEntry {
            character_level: 700,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = BreakinPoolQuery {
            character_level: 400,
            weapon_level: 1,
        };

        assert!(BreakinHostMatcher::matches(&host, &invader) == true);
    }
}
