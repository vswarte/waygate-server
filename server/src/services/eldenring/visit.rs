use std::{
    collections::HashMap,
    sync::{mpsc::Sender, LazyLock, Mutex, MutexGuard},
};

use dashmap::DashMap;
use message::eldenring::VisitType;

use crate::{logging::LogContext, services::eldenring::PoolError};

use super::weapon;

/// Pool for summon signs. Both coop and duelist.
#[derive(Default)]
pub struct VisitorPool {
    entries: DashMap<VisitorPoolKey, VisitorPoolEntry>,
}

impl VisitorPool {
    pub fn insert(&self, player_id: i32, entry: VisitorPoolEntry) -> VisitorPoolToken {
        let key = VisitorPoolKey(player_id);
        self.entries.insert(key.clone(), entry);
        VisitorPoolToken(self, key)
    }

    pub fn get(&self, key: &VisitorPoolKey) -> Option<VisitorPoolEntry> {
        self.entries.get(key).map(|i| i.clone())
    }

    pub fn matches(&self, query: &VisitorPoolQuery) -> Vec<(VisitorPoolKey, VisitorPoolEntry)> {
        self.entries
            .iter()
            .filter(|e| query.matches(e.value()))
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect()
    }

    pub fn remove(&self, key: &VisitorPoolKey) -> Result<(), PoolError> {
        self.entries.remove(key).ok_or(PoolError::NotFound)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct VisitorPoolEntry {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub play_region: u32,
    pub visit_type: VisitType,
    pub external_id: String,
    pub visitor_tx: Sender<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub struct VisitorPoolQuery {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub play_region: u32,
    pub visit_type: VisitType,
}

impl VisitorPoolQuery {
    fn matches(&self, entry: &VisitorPoolEntry) -> bool {
        if entry.player_id == self.player_id {
            return false;
        }
        entry.play_region == self.play_region
            && entry.visit_type == self.visit_type
            && Self::check_character_level(entry.character_level, self.character_level)
            && Self::check_weapon_level(entry.weapon_level, self.weapon_level)
    }

    fn check_character_level(visitor: u32, host: u32) -> bool {
        let lower = host - (host / 10);
        let upper = host + (host / 10) + 20;

        if host >= 301 {
            visitor >= lower
        } else {
            visitor >= lower && visitor <= upper
        }
    }

    fn check_weapon_level(visitor: u32, host: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(visitor) {
            entry.regular_range.contains(&host)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitorPoolKey(pub i32);

/// Represents an entry in the sign pool. Removes corresponding entry when dropped.
pub struct VisitorPoolToken<'a>(&'a VisitorPool, pub VisitorPoolKey);

impl Drop for VisitorPoolToken<'_> {
    fn drop(&mut self) {
        let _ = self.0.remove(&self.1);
    }
}

pub static VISIT_ATTEMPTS: LazyLock<VisitorAttemptTracker> = LazyLock::new(Default::default);

#[derive(Clone)]
pub struct VisitorAttempt {
    /// Summoners notification channel
    pub summoner_tx: Sender<Vec<u8>>,
}

#[derive(Default)]
pub struct VisitorAttemptTracker {
    entries: Mutex<HashMap<(VisitorPoolKey, i32), VisitorAttempt>>,
}

impl VisitorAttemptTracker {
    pub fn insert(&'static self, key: (VisitorPoolKey, i32), attempt: VisitorAttempt) {
        self.lock().insert(key, attempt);
    }

    pub fn remove(&self, key: &(VisitorPoolKey, i32)) -> Option<VisitorAttempt> {
        self.lock().remove(key)
    }

    fn lock(&self) -> MutexGuard<'_, HashMap<(VisitorPoolKey, i32), VisitorAttempt>> {
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

    use message::eldenring::VisitType;

    use super::{VisitorPoolEntry, VisitorPoolQuery};

    #[test]
    fn test_character_level() {
        assert!(VisitorPoolQuery::check_character_level(700, 400));
        assert!(!VisitorPoolQuery::check_character_level(1, 713));
        assert!(VisitorPoolQuery::check_character_level(28, 31));
        assert!(VisitorPoolQuery::check_character_level(54, 31));
    }

    #[test]
    fn test_weapon_level() {
        assert!(VisitorPoolQuery::check_weapon_level(0, 0));
        assert!(VisitorPoolQuery::check_weapon_level(0, 2));
        assert!(!VisitorPoolQuery::check_weapon_level(0, 4));
        assert!(VisitorPoolQuery::check_weapon_level(12, 14));
        assert!(VisitorPoolQuery::check_weapon_level(12, 8));
        assert!(!VisitorPoolQuery::check_weapon_level(12, 25));
    }

    #[test]
    fn level_1_characters_match() {
        let (visitor_tx, _) = channel();
        let visitor = VisitorPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
            external_id: String::default(),
            visitor_tx,
        };

        let host = VisitorPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
        };

        assert!(host.matches(&visitor));
    }

    #[test]
    fn level_fall_off_applies() {
        let (visitor_tx, _) = channel();
        let visitor = VisitorPoolEntry {
            player_id: 1,
            character_level: 700,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
            external_id: String::default(),
            visitor_tx,
        };

        let host = VisitorPoolQuery {
            player_id: 2,
            character_level: 400,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
        };

        assert!(host.matches(&visitor));
    }

    #[test]
    fn play_region_must_match() {
        let (visitor_tx, _) = channel();
        let visitor = VisitorPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 1,
            visit_type: VisitType::Hunter,
            external_id: String::default(),
            visitor_tx,
        };

        let host = VisitorPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
        };

        assert!(!host.matches(&visitor));
    }

    #[test]
    fn self_match_fails() {
        let (visitor_tx, _) = channel();
        let visitor = VisitorPoolEntry {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
            external_id: String::default(),
            visitor_tx,
        };

        let host = VisitorPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
        };

        assert!(!host.matches(&visitor));
    }
}
