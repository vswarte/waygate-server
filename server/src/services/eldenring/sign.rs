use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, Ordering},
        mpsc::Sender,
        LazyLock, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
    time::Duration,
};

use message::eldenring::PlayRegionArea;

use crate::services::eldenring::PoolError;

use super::weapon;

pub const SIGN_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);

/// Pool for summon signs. Both coop and duelist.
#[derive(Default)]
pub struct SignPool {
    counter: AtomicI64,
    entries: RwLock<HashMap<SignPoolKey, SignPoolEntry>>,
}

impl SignPool {
    pub fn insert(&self, entry: SignPoolEntry) -> SignPoolToken {
        let key = SignPoolKey(self.counter.fetch_add(1, Ordering::Relaxed));
        self.lock_write().insert(key.clone(), entry);
        SignPoolToken(self, key)
    }

    pub fn get(&self, key: &SignPoolKey) -> Option<SignPoolEntry> {
        self.lock_read().get(key).cloned()
    }

    pub fn has(&self, key: &SignPoolKey) -> bool {
        self.lock_read().contains_key(key)
    }

    pub fn matches(&self, query: &SignPoolQuery) -> Vec<(SignPoolKey, SignPoolEntry)> {
        self.lock_read()
            .iter()
            .filter(|e| query.matches(e.1))
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    pub fn matches_puddle(&self, query: &PuddleSignPoolQuery) -> Vec<(SignPoolKey, SignPoolEntry)> {
        self.lock_read()
            .iter()
            .filter(|e| query.matches(e.1))
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    pub fn remove(&self, key: &SignPoolKey) -> Result<(), PoolError> {
        self.lock_write().remove(key).ok_or(PoolError::NotFound)?;
        Ok(())
    }

    fn lock_read(&self) -> RwLockReadGuard<'_, HashMap<SignPoolKey, SignPoolEntry>> {
        self.entries.read().unwrap_or_else(|p| {
            log::warn!("Sign pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }

    fn lock_write(&self) -> RwLockWriteGuard<'_, HashMap<SignPoolKey, SignPoolEntry>> {
        self.entries.write().unwrap_or_else(|p| {
            log::warn!("Sign pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }
}

#[derive(Clone, Debug)]
pub struct SignPoolEntry {
    pub player_id: i32,
    pub external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub location: SignLocation,
    pub password: String,
    pub group_passwords: Vec<String>,
    pub data: Vec<u8>,
    pub summonee_tx: Sender<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SignLocation {
    Puddle(i32),
    Area(PlayRegionArea),
}

#[derive(Clone, Debug)]
pub struct SignPoolQuery<'a> {
    pub character_level: u32,
    pub weapon_level: u32,
    pub areas: &'a [PlayRegionArea],
    pub password: &'a str,
}

impl SignPoolQuery<'_> {
    pub fn matches(&self, entry: &SignPoolEntry) -> bool {
        if !self
            .areas
            .iter()
            .map(|a| SignLocation::Area((*a).clone()))
            .collect::<Vec<_>>()
            .contains(&entry.location)
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

#[derive(Clone, Debug)]
pub struct PuddleSignPoolQuery<'a> {
    pub character_level: u32,
    pub weapon_level: u32,
    pub puddle: i32,
    pub password: &'a str,
}

impl PuddleSignPoolQuery<'_> {
    pub fn matches(&self, entry: &SignPoolEntry) -> bool {
        if SignLocation::Puddle(self.puddle) != entry.location {
            return false;
        }

        if !entry.password.is_empty() || !self.password.is_empty() {
            return entry.password.eq(&self.password);
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SignPoolKey(pub i64);

/// Represents an entry in the sign pool. Removes corresponding entry when dropped.
pub struct SignPoolToken<'a>(&'a SignPool, pub SignPoolKey);

impl Drop for SignPoolToken<'_> {
    fn drop(&mut self) {
        let _ = self.0.remove(&self.1);
    }
}

pub static SUMMON_ATTEMPTS: LazyLock<SummonAttemptTracker> = LazyLock::new(Default::default);

#[derive(Clone)]
pub struct SummonAttempt {
    /// Summoner channel sender for push notifs
    pub summoner_tx: Sender<Vec<u8>>,
}

#[derive(Default)]
pub struct SummonAttemptTracker {
    entries: Mutex<HashMap<(SignPoolKey, i32), SummonAttempt>>,
}

impl SummonAttemptTracker {
    pub fn insert(&'static self, key: (SignPoolKey, i32), attempt: SummonAttempt) {
        self.lock().insert(key, attempt);
    }

    pub fn remove(&self, key: &(SignPoolKey, i32)) -> Option<SummonAttempt> {
        self.lock().remove(key)
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<(SignPoolKey, i32), SummonAttempt>> {
        self.entries.lock().unwrap_or_else(|p| {
            log::warn!("Sign pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use message::eldenring::PlayRegionArea;

    use crate::services::eldenring::sign::SignLocation;

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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::from("test"),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::from("123"),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
            location: SignLocation::Area(PlayRegionArea {
                area: 1,
                play_region: 1,
            }),
            password: String::default(),
            group_passwords: vec![],
            data: vec![],
            summonee_tx,
        };

        let finger = SignPoolQuery {
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
}
