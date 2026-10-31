use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        mpsc::Sender,
    },
    time::Duration,
};

use message::eldenring::{ObjectIdentifier, PlayRegionArea, PuddleArea};

use crate::services::eldenring::area::MatchingArea;
use crate::services::ordered_pool::{Keyed, OrderedPool, OrderedPoolToken, PoolError};

use super::weapon;

#[derive(Default)]
pub struct SignPool {
    counter: AtomicI64,
    inner: OrderedPool<SignPoolEntry>,
}

impl SignPool {
    pub fn insert(&self, mut entry: SignPoolEntry) -> SignPoolToken<'_> {
        entry.id = ObjectIdentifier(self.counter.fetch_add(1, Ordering::Relaxed));
        self.inner.insert(entry)
    }

    pub fn get(&self, key: &ObjectIdentifier) -> Option<SignPoolEntry> {
        self.inner.get(key)
    }

    pub fn touch(&self, key: &ObjectIdentifier) -> Result<(), PoolError> {
        self.inner.merge(key, |_| {})
    }

    pub fn matches(&self, query: &SignPoolQuery) -> Vec<SignPoolEntry> {
        self.inner.matches_all(|e| query.matches(e))
    }

    pub fn matches_puddle(&self, query: &PuddleSignPoolQuery) -> Vec<SignPoolEntry> {
        self.inner.matches_all(|e| query.matches(e))
    }

    pub fn sweep_expired(&self, ttl: Duration) {
        self.inner.sweep_expired(ttl);
    }
}

#[derive(Clone, Debug)]
pub struct SignPoolEntry {
    pub id: ObjectIdentifier,
    pub player_id: i32,
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub location: MatchingArea,
    pub password: String,
    pub group_passwords: Vec<String>,
    pub data: Vec<u8>,
    pub summonee_tx: Sender<Vec<u8>>,
    pub updated_at: tokio::time::Instant,
}

impl Keyed for SignPoolEntry {
    type Key = ObjectIdentifier;

    fn key(&self) -> ObjectIdentifier {
        self.id
    }

    fn updated_at(&self) -> tokio::time::Instant {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = tokio::time::Instant::now();
    }
}

#[derive(Clone, Debug)]
pub struct SignPoolQuery<'a> {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub areas: &'a [PlayRegionArea],
    pub password: &'a str,
}

impl SignPoolQuery<'_> {
    pub fn matches(&self, entry: &SignPoolEntry) -> bool {
        if entry.player_id == self.player_id {
            return false;
        }
        if !self
            .areas
            .iter()
            .any(|a| entry.location.matches(&MatchingArea::PlayRegion(*a)))
        {
            return false;
        }

        if !entry.password.is_empty() || !self.password.is_empty() {
            return entry.password.eq(&self.password);
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }

    fn check_character_level(host: u32, finger: u32) -> bool {
        let radius = (host as f32 * 0.1).ceil() as u32 + 10;
        let lower = host.saturating_sub(radius);

        let upper = if finger >= 306 { 713 } else { host + radius };

        (lower..=upper).contains(&finger)
    }

    fn check_weapon_level(host: u32, finger: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&finger)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuddleSignPoolQuery<'a> {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub puddles: Vec<PuddleArea>,
    pub password: &'a str,
}

impl PuddleSignPoolQuery<'_> {
    pub fn matches(&self, entry: &SignPoolEntry) -> bool {
        if entry.player_id == self.player_id {
            return false;
        }
        if !self
            .puddles
            .iter()
            .any(|puddle| MatchingArea::Puddle(puddle.clone()).matches(&entry.location))
        {
            return false;
        }

        if !entry.password.is_empty() || !self.password.is_empty() {
            return entry.password.eq(&self.password);
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }

    fn check_character_level(host: u32, finger: u32) -> bool {
        let radius = (host as f32 * 0.1).ceil() as u32 + 10;
        let lower = host.saturating_sub(radius);

        let upper = if finger >= 306 { 713 } else { host + radius };

        (lower..=upper).contains(&finger)
    }

    fn check_weapon_level(host: u32, finger: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&finger)
        } else {
            false
        }
    }
}

pub type SignPoolToken<'a> = OrderedPoolToken<'a, SignPoolEntry>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SummonAttemptKey {
    pub sign_id: ObjectIdentifier,
    pub summoner_id: i32,
}

#[derive(Clone)]
pub struct SummonAttempt {
    /// Summoner channel sender for push notifs
    pub summoner_tx: Sender<Vec<u8>>,
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use message::eldenring::PlayRegionArea;

    use crate::services::eldenring::sign::MatchingArea;

    use super::{SignPoolEntry, SignPoolQuery};

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
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            external_id: String::new(),
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "",
        };

        assert!(finger.matches(&host));
    }

    #[test]
    fn doesnt_match_differing_levels() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 100,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "",
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn password_matches_regardless() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::from("test"),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 713,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "test",
        };

        assert!(finger.matches(&host));
    }

    #[test]
    fn doesnt_match_on_differing_passwords() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::from("123"),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "456",
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn doesnt_match_when_password_isnt_set_on_host() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "456",
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn doesnt_match_across_search_areas() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 2,
                play_region: 2,
            }],
            password: "",
        };

        assert!(!finger.matches(&host));
    }

    #[test]
    fn self_match_fails() {
        let (summonee_tx, _) = channel();
        let host = SignPoolEntry {
            player_id: 1,
            external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            location: MatchingArea::PlayRegion(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
            id: Default::default(),
            updated_at: tokio::time::Instant::now(),
        };

        let finger = SignPoolQuery {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            areas: &[PlayRegionArea {
                area: 1,
                play_region: 1,
            }],
            password: "",
        };

        assert!(!finger.matches(&host));
    }
}
