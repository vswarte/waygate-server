use super::pool::PoolEntryMatcher;
use crate::pool::matching::weapon;

#[derive(Clone, Debug)]
pub struct InvasionPoolEntry {
    pub character_level: u16,
    pub weapon_level: u16,
    pub steam_id: String,
}

#[derive(Debug)]
pub struct InvasionPoolQuery {
    pub character_level: u16,
    pub weapon_level: u16,
}

// TODO: give a wet shit about invasion areas
pub struct InvasionHostMatcher;

impl InvasionHostMatcher {
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

impl PoolEntryMatcher<InvasionPoolEntry, InvasionPoolQuery> for InvasionHostMatcher {
    fn matches(entry: &InvasionPoolEntry, query: &InvasionPoolQuery) -> bool {
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
    use crate::pool::invasion::{InvasionPoolEntry, InvasionPoolQuery, InvasionHostMatcher};

    #[test]
    fn test_character_level() {
        assert!(InvasionHostMatcher::check_character_level(700, 400) == true);
        assert!(InvasionHostMatcher::check_character_level(1, 713) == false);
        assert!(InvasionHostMatcher::check_character_level(28, 31) == true);
        assert!(InvasionHostMatcher::check_character_level(54, 31) == true);
    }

    #[test]
    fn test_weapon_level() {
        assert!(InvasionHostMatcher::check_weapon_level(0, 0) == true);
        assert!(InvasionHostMatcher::check_weapon_level(0, 2) == true);
        assert!(InvasionHostMatcher::check_weapon_level(0, 3) == false);
        assert!(InvasionHostMatcher::check_weapon_level(12, 14) == true);
        assert!(InvasionHostMatcher::check_weapon_level(12, 8) == true);
        assert!(InvasionHostMatcher::check_weapon_level(12, 25) == false);
    }

    #[test]
    fn level_1_characters_match() {
        let host = InvasionPoolEntry {
            character_level: 1,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = InvasionPoolQuery {
            character_level: 1,
            weapon_level: 1,
        };

        assert!(InvasionHostMatcher::matches(&host, &invader) == true);
    }

    #[test]
    fn level_fall_off_applies() {
        let host = InvasionPoolEntry {
            character_level: 700,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = InvasionPoolQuery {
            character_level: 400,
            weapon_level: 1,
        };

        assert!(InvasionHostMatcher::matches(&host, &invader) == true);
    }
}
