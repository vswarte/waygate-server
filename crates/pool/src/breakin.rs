use waygate_message::{RequestGetBreakInTargetListParams, ResponseGetBreakInTargetListParamsEntry};

use crate::{MatchResult, PoolQuery};
use crate::matching::weapon;

#[derive(Debug, Default)]
pub struct BreakInPoolEntry {
    pub character_level: u32,
    pub weapon_level: u32,
    pub steam_id: String,
}

impl Into<ResponseGetBreakInTargetListParamsEntry> for &MatchResult<BreakInPoolEntry> {
    fn into(self) -> ResponseGetBreakInTargetListParamsEntry {
        ResponseGetBreakInTargetListParamsEntry {
            player_id: self.0.1,
            steam_id: self.1.steam_id.clone(),
        }
    }
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

impl From<RequestGetBreakInTargetListParams> for BreakInPoolQuery {
    fn from(value: RequestGetBreakInTargetListParams) -> Self {
        BreakInPoolQuery {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::breakin::PoolQuery;
    use crate::breakin::{BreakInPoolEntry, BreakInPoolQuery};

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
        assert!(!BreakInPoolQuery::check_weapon_level(0, 4));
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
