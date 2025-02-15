use std::{
    collections::HashMap,
    sync::{mpsc::Sender, LazyLock, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::services::eldenring::PoolError;

use super::weapon;

/// Pool for summon signs. Both coop and duelist.
#[derive(Default)]
pub struct QuickMatchPool {
    entries: RwLock<HashMap<QuickMatchPoolKey, QuickMatchPoolEntry>>,
}

impl QuickMatchPool {
    pub fn insert(
        &self,
        player_id: i32,
        entry: QuickMatchPoolEntry,
    ) -> QuickMatchPoolToken {
        let key = QuickMatchPoolKey(player_id);
        self.lock_write().insert(key.clone(), entry);
        QuickMatchPoolToken(self, key)
    }

    pub fn get(&self, key: &QuickMatchPoolKey) -> Option<QuickMatchPoolEntry> {
        self.lock_read().get(key).cloned()
    }

    pub fn matches(
        &self,
        query: &QuickMatchPoolQuery,
    ) -> Vec<(QuickMatchPoolKey, QuickMatchPoolEntry)> {
        self.lock_read()
            .iter()
            .filter(|e| query.matches(e.1))
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }

    pub fn remove(&self, key: &QuickMatchPoolKey) -> Result<(), PoolError> {
        self.lock_write().remove(key).ok_or(PoolError::NotFound)?;
        Ok(())
    }

    pub fn merge(&self, key: &QuickMatchPoolKey, merger: impl Fn(&mut QuickMatchPoolEntry)) -> Result<(), PoolError> {
        match self.lock_write().get_mut(key) {
            Some(e) => merger(e),
            None => return Err(PoolError::NotFound),
        };
        Ok(())
    }

    fn lock_read(&self) -> RwLockReadGuard<'_, HashMap<QuickMatchPoolKey, QuickMatchPoolEntry>> {
        self.entries.read().unwrap_or_else(|p| {
            log::warn!("QuickMatch pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }

    fn lock_write(&self) -> RwLockWriteGuard<'_, HashMap<QuickMatchPoolKey, QuickMatchPoolEntry>> {
        self.entries.write().unwrap_or_else(|p| {
            log::warn!("QuickMatch pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QuickMatchPoolKey(pub i32);

/// Represents an entry in the sign pool. Removes corresponding entry when dropped.
pub struct QuickMatchPoolToken<'a>(&'a QuickMatchPool, pub QuickMatchPoolKey);

impl Drop for QuickMatchPoolToken<'_> {
    fn drop(&mut self) {
        let _ = self.0.remove(&self.1);
    }
}

#[derive(Clone, Debug)]
pub struct QuickMatchPoolEntry {
    pub host_player_id: i32,
    pub host_external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub arena_id: i32,
    pub password: String,
    pub quickmatch_settings: i32,
    pub host_tx: Sender<Vec<u8>>,
}

#[derive(Debug)]
pub struct QuickMatchPoolQuery {
    pub arena_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub password: String,
    pub quickmatch_settings: i32,
}

impl QuickMatchPoolQuery {
    fn matches(&self, entry: &QuickMatchPoolEntry) -> bool {
        // TODO: password team matching
        if self.arena_id != entry.arena_id || self.quickmatch_settings != entry.quickmatch_settings {
            return false;
        }

        if !entry.password.is_empty() || !self.password.is_empty() {
            return entry.password.eq(&self.password);
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }

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

pub static QUICKMATCH_JOIN_ATTEMPTS: LazyLock<QuickMatchJoinAttemptTracker> =
    LazyLock::new(Default::default);

#[derive(Clone)]
pub struct QuickMatchJoinAttempt {
    pub quickmatch_settings: i32,
    /// Summoner channel sender for push notifs
    pub joining_player_tx: Sender<Vec<u8>>,
}

#[derive(Default)]
pub struct QuickMatchJoinAttemptTracker {
    entries: Mutex<HashMap<(QuickMatchPoolKey, i32), QuickMatchJoinAttempt>>,
}

impl QuickMatchJoinAttemptTracker {
    pub fn insert(&'static self, key: (QuickMatchPoolKey, i32), attempt: QuickMatchJoinAttempt) {
        self.lock().insert(key, attempt);
    }

    pub fn remove(&self, key: &(QuickMatchPoolKey, i32)) -> Option<QuickMatchJoinAttempt> {
        self.lock().remove(key)
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<(QuickMatchPoolKey, i32), QuickMatchJoinAttempt>> {
        self.entries.lock().unwrap_or_else(|p| {
            log::warn!("QuickMatch lobby pool recovering from mutex poisoning");
            self.entries.clear_poison();
            p.into_inner()
        })
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use super::{QuickMatchPoolEntry, QuickMatchPoolQuery};

    #[test]
    fn test_character_level() {
        assert!(!QuickMatchPoolQuery::check_character_level(1, 300));
        assert!(QuickMatchPoolQuery::check_character_level(28, 31));
    }

    #[test]
    fn test_weapon_level() {
        assert!(QuickMatchPoolQuery::check_weapon_level(0, 0));
        assert!(QuickMatchPoolQuery::check_weapon_level(0, 2));
        assert!(!QuickMatchPoolQuery::check_weapon_level(0, 4));
        assert!(QuickMatchPoolQuery::check_weapon_level(12, 14));
        assert!(QuickMatchPoolQuery::check_weapon_level(12, 8));
        assert!(!QuickMatchPoolQuery::check_weapon_level(12, 25));
    }

    #[test]
    fn level_1_characters_match() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_differing_levels() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 100,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn password_matches_regardless() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::from("test"),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 713,
            weapon_level: 1,
            password: String::from("test"),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_on_differing_passwords() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::from("123"),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::from("456"),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_when_password_isnt_set_on_joiner() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::from("456"),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_mismatching_arena_ids() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x1,
            quickmatch_settings: 0x0,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn doesnt_match_mismatching_settings() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 1,
            weapon_level: 1,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x1,
        };

        assert!(!joiner.matches(&host));
    }

    #[test]
    fn test_clayamore_group() {
        let (host_tx, _) = channel();
        let host = QuickMatchPoolEntry {
            host_player_id: 1,
            host_external_id: String::new(),
            character_level: 130,
            weapon_level: 25,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
            host_tx,
        };

        let joiner = QuickMatchPoolQuery {
            character_level: 137,
            weapon_level: 25,
            password: String::default(),
            arena_id: 0x0,
            quickmatch_settings: 0x0,
        };

        assert!(joiner.matches(&host));
    }
}
