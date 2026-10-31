use std::{sync::mpsc::Sender, time::Duration};

use message::eldenring::VisitType;

use crate::services::ordered_pool::{Keyed, OrderedPool, OrderedPoolToken};

use super::weapon;

#[derive(Default)]
pub struct VisitorPool {
    inner: OrderedPool<VisitorPoolEntry>,
}

impl VisitorPool {
    pub fn sync<'a>(
        &'a self,
        token: &mut Option<VisitorPoolToken<'a>>,
        entry: Option<VisitorPoolEntry>,
    ) {
        self.inner.sync(token, entry)
    }

    pub fn sweep_expired(&self, ttl: Duration) {
        self.inner.sweep_expired(ttl);
    }

    pub fn get(&self, key: &VisitorPoolKey) -> Option<VisitorPoolEntry> {
        self.inner.get(key)
    }

    pub fn matches(&self, query: &VisitorPoolQuery) -> Vec<VisitorPoolEntry> {
        self.inner.matches_all(|e| query.matches(e))
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
    pub updated_at: tokio::time::Instant,
}

impl Keyed for VisitorPoolEntry {
    type Key = VisitorPoolKey;

    fn key(&self) -> VisitorPoolKey {
        VisitorPoolKey {
            player_id: self.player_id,
            visit_type: self.visit_type,
        }
    }

    fn updated_at(&self) -> tokio::time::Instant {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = tokio::time::Instant::now();
    }
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
        let lower = host.saturating_sub(host / 10);

        let upper = if host >= 301 {
            713
        } else {
            host + (host as f32 * 0.1).ceil() as u32 + 20
        };

        (lower..=upper).contains(&visitor)
    }

    fn check_weapon_level(visitor: u32, host: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&visitor)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitorPoolKey {
    pub player_id: i32,
    pub visit_type: VisitType,
}

pub type VisitorPoolToken<'a> = OrderedPoolToken<'a, VisitorPoolEntry>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitAttemptKey {
    pub visitor_id: VisitorPoolKey,
    pub summoner_id: i32,
}

#[derive(Clone)]
pub struct VisitorAttempt {
    /// Summoners notification channel
    pub summoner_tx: Sender<Vec<u8>>,
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
            updated_at: tokio::time::Instant::now(),
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
            updated_at: tokio::time::Instant::now(),
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
            updated_at: tokio::time::Instant::now(),
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
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            visit_type: VisitType::Hunter,
            external_id: String::default(),
            visitor_tx,
            updated_at: tokio::time::Instant::now(),
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
