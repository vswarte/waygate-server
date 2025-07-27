use std::{
    collections::HashMap,
    sync::{mpsc::Sender, LazyLock, Mutex, MutexGuard},
    time::Duration,
};

use dashmap::DashMap;

use crate::{logging::LogContext, services::eldenring::PoolError};

use super::weapon;

pub const BREAKIN_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);

// TODO: in case of far breakins the client will send a request **per area**. This leads to a lot
// of locking. It might be better to split out these pools per area.
#[derive(Default)]
pub struct BreakInPool {
    entries: DashMap<BreakInPoolKey, BreakInPoolEntry>,
}

impl BreakInPool {
    pub fn insert(&self, player_id: i32, entry: BreakInPoolEntry) -> BreakInPoolToken {
        let key = BreakInPoolKey(player_id);
        self.entries.insert(key.clone(), entry);
        BreakInPoolToken(self, key)
    }

    pub fn get(&self, key: &BreakInPoolKey) -> Option<BreakInPoolEntry> {
        self.entries.get(key).map(|e| e.clone())
    }

    pub fn matches(&self, query: &BreakInPoolQuery) -> Vec<(BreakInPoolKey, BreakInPoolEntry)> {
        self.entries
            .iter()
            .filter(|e| query.matches(e.value()))
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect()
    }

    pub fn remove(&self, key: &BreakInPoolKey) -> Result<(), PoolError> {
        self.entries.remove(key).ok_or(PoolError::NotFound)?;
        Ok(())
    }

    pub fn replace(
        &self,
        key: &BreakInPoolKey,
        entry: BreakInPoolEntry,
    ) -> Result<BreakInPoolEntry, PoolError> {
        self.entries
            .insert(key.clone(), entry)
            .ok_or(PoolError::NotFound)
    }
}

#[derive(Clone, Debug)]
pub struct BreakInPoolEntry {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub play_region: u32,
    pub external_id: String,
    pub target_tx: Sender<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct BreakInPoolQuery {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub play_region: u32,
}

impl BreakInPoolQuery {
    fn matches(&self, entry: &BreakInPoolEntry) -> bool {
        if entry.player_id == self.player_id {
            return false;
        }
        entry.play_region == self.play_region
            && Self::check_character_level(entry.character_level, self.character_level)
            && Self::check_weapon_level(entry.weapon_level, self.weapon_level)
    }

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BreakInPoolKey(pub i32);

/// Represents an entry in the sign pool. Removes corresponding entry when dropped.
pub struct BreakInPoolToken<'a>(&'a BreakInPool, pub BreakInPoolKey);

impl Drop for BreakInPoolToken<'_> {
    fn drop(&mut self) {
        let _ = self.0.remove(&self.1);
    }
}

pub static BREAKIN_ATTEMPTS: LazyLock<BreakInAttemptTracker> = LazyLock::new(Default::default);

#[derive(Clone)]
pub struct BreakInAttempt {
    /// invader channel sender for push notifs
    pub invader_tx: Sender<Vec<u8>>,
}

#[derive(Default)]
pub struct BreakInAttemptTracker {
    entries: Mutex<HashMap<(BreakInPoolKey, i32), BreakInAttempt>>,
}

impl BreakInAttemptTracker {
    pub fn insert(&'static self, key: (BreakInPoolKey, i32), attempt: BreakInAttempt) {
        self.lock().insert(key, attempt);
    }

    pub fn remove(&self, key: &(BreakInPoolKey, i32)) -> Option<BreakInAttempt> {
        self.lock().remove(key)
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<(BreakInPoolKey, i32), BreakInAttempt>> {
        self.entries.lock().unwrap_or_else(|p| {
            log::warn!(
                context:serde = LogContext::current();
                "Sign pool recovering from mutex poisoning"
            );
            self.entries.clear_poison();
            p.into_inner()
        })
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;

    use super::{BreakInPoolEntry, BreakInPoolQuery};

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
        let (target_tx, _) = channel();
        let host = BreakInPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            external_id: String::default(),
            target_tx,
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
        };

        assert!(invader.matches(&host));
    }

    #[test]
    fn level_fall_off_applies() {
        let (target_tx, _) = channel();
        let host = BreakInPoolEntry {
            player_id: 1,
            character_level: 700,
            weapon_level: 1,
            play_region: 0,
            external_id: String::default(),
            target_tx,
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 400,
            weapon_level: 1,
            play_region: 0,
        };

        assert!(invader.matches(&host));
    }

    #[test]
    fn play_region_must_match() {
        let (target_tx, _) = channel();
        let host = BreakInPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 1,
            external_id: String::default(),
            target_tx,
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
        };

        assert!(!invader.matches(&host));
    }

    #[test]
    fn self_match_fails() {
        let (target_tx, _) = channel();
        let host = BreakInPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            external_id: String::default(),
            target_tx,
        };

        let invader = BreakInPoolQuery {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
        };

        assert!(!invader.matches(&host));
    }
}
