use waygate_message::{RequestCreateMatchAreaSignParams, RequestCreateSignParams, RequestGetMatchAreaSignListParams, RequestGetSignListParams, ResponseGetMatchAreaSignListParamsEntry, ResponseGetSignListParamsEntry};

use crate::{MatchResult, PoolQuery};
use crate::matching::weapon;
use crate::matching::area::MatchingArea;

#[derive(Debug, Default)]
pub struct SignPoolEntry {
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub area: MatchingArea,
    pub unk_area_value: i32,
    pub password: String,
    pub group_passwords: Vec<String>,
    pub data: Vec<u8>,
}

impl From<RequestCreateSignParams> for SignPoolEntry {
    fn from(val: RequestCreateSignParams) -> Self {
        SignPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
            area: (&val.area).into(),
            password: val.matching_parameters.password.clone(),
            group_passwords: val.group_passwords.clone(),
            data: val.data,
            unk_area_value: 0,
        }
    }
}

impl From<RequestCreateMatchAreaSignParams> for SignPoolEntry {
    fn from(val: RequestCreateMatchAreaSignParams) -> Self {
        SignPoolEntry {
            external_id: String::new(),
            character_level: val.matching_parameters.soul_level as u32,
            weapon_level: val.matching_parameters.max_reinforce as u32,
            area: (&val.area.match_area).into(),
            password: val.matching_parameters.password.clone(),
            group_passwords: val.group_passwords.clone(),
            data: val.data,
            unk_area_value: val.area.area,
        }
    }
}

impl Into<ResponseGetSignListParamsEntry> for &MatchResult<SignPoolEntry> {
    fn into(self) -> ResponseGetSignListParamsEntry {
        ResponseGetSignListParamsEntry {
            player_id: self.0.1,
            identifier: (&self.0).into(),
            area: (&self.1.area).into(),
            data: self.1.data.clone(),
            steam_id: self.1.external_id.clone(),
            unk_string: String::default(),
            group_passwords: self.1.group_passwords.clone(),
        }

    }
}

impl From<&MatchResult<SignPoolEntry>> for ResponseGetMatchAreaSignListParamsEntry {
    fn from(val: &MatchResult<SignPoolEntry>) -> Self {
        ResponseGetMatchAreaSignListParamsEntry {
            player_id: val.0.1,
            identifier: (&val.0).into(),
            data: val.1.data.clone(),
            steam_id: val.1.external_id.clone(),
            area: (&val.1.area).into(),
            unk1: val.1.unk_area_value,
            unk2: 0,
            unk_string: String::default(),
            group_passwords: val.1.group_passwords.clone(),
        }
    }
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

impl From<&RequestGetSignListParams> for SignPoolQuery {
    fn from(value: &RequestGetSignListParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            areas: value.search_areas.iter().map(|e| e.into()).collect(),
            password: value.matching_parameters.password.clone(),
        }
    }
}

impl From<&RequestGetMatchAreaSignListParams> for SignPoolQuery {
    fn from(value: &RequestGetMatchAreaSignListParams) -> Self {
        Self {
            character_level: value.matching_parameters.soul_level as u32,
            weapon_level: value.matching_parameters.max_reinforce as u32,
            areas: vec![MatchingArea::Puddle(value.match_area_id)],
            password: value.matching_parameters.password.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PoolQuery;
    use crate::matching::area::MatchingArea;
    use crate::sign::{SignPoolEntry, SignPoolQuery};

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
            unk_area_value: 0,
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
            unk_area_value: 0,
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
            unk_area_value: 0,
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
            unk_area_value: 0,
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
            unk_area_value: 0,
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
            unk_area_value: 0,
        };

        let finger = SignPoolQuery {
            character_level: 1,
            weapon_level: 1,
            areas: vec![MatchingArea::PlayRegion { area: 2, play_region: 2}],
            password: String::default(),
        };

        assert!(!finger.matches(&host));
    }
}
