use super::PoolQuery;
use crate::pool::matching::weapon;

#[derive(Clone, Debug)]
pub struct BreakInPoolEntry {
    pub character_level: u32,
    pub weapon_level: u32,
    pub steam_id: String,
}

#[derive(Debug)]
pub struct BreakInPoolQuery {
    pub character_level: u32,
    pub weapon_level: u32,
}

impl BreakInPoolQuery {
    fn check_character_level(host: u32, invader: u32) -> bool {
        let lower = invader - (invader / 10);
        let upper = invader + (invader / 10) + 20;

        if invader >= 301 {
            host >= lower
        } else {
            host >= lower && host <= upper
        }
    }

    fn check_weapon_level(host: u32, invader: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&invader)
        } else {
            false
        }
    }
}

impl PoolQuery<BreakInPoolEntry> for BreakInPoolQuery {
    fn matches(&self, entry: &BreakInPoolEntry) -> bool {
        Self::check_character_level(
            entry.character_level,
            self.character_level,
        ) && Self::check_weapon_level(
            entry.weapon_level,
            self.weapon_level,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::pool::{breakin::{BreakInPoolEntry, BreakInPoolQuery}, PoolQuery};

    #[test]
    fn test_character_level() {
        assert!(BreakInPoolQuery::check_character_level(700, 400));
        assert!(!BreakInPoolQuery::check_character_level(1, 713));
        assert!(BreakInPoolQuery::check_character_level(28, 31));
        assert!(BreakInPoolQuery::check_character_level(54, 31));
    }

    #[test]
    fn test_weapon_level() {
        assert!(BreakInPoolQuery::check_weapon_level(0, 0));
        assert!(BreakInPoolQuery::check_weapon_level(0, 2));
        assert!(!BreakInPoolQuery::check_weapon_level(0, 3));
        assert!(BreakInPoolQuery::check_weapon_level(12, 14));
        assert!(BreakInPoolQuery::check_weapon_level(12, 8));
        assert!(!BreakInPoolQuery::check_weapon_level(12, 25));
    }

    #[test]
    fn level_1_characters_match() {
        let host = BreakInPoolEntry {
            character_level: 1,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = BreakInPoolQuery {
            character_level: 1,
            weapon_level: 1,
        };

        assert!(invader.matches(&host));
    }

    #[test]
    fn level_fall_off_applies() {
        let host = BreakInPoolEntry {
            character_level: 700,
            weapon_level: 1,
            steam_id: String::default(),
        };

        let invader = BreakInPoolQuery {
            character_level: 400,
            weapon_level: 1,
        };

        assert!(invader.matches(&host));
    }
}
