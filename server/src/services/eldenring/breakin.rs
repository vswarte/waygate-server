use std::{sync::mpsc::Sender, time::Duration};

use crate::services::ordered_pool::{Keyed, Order, OrderedPool, OrderedPoolToken};

use super::weapon;

#[derive(Default)]
pub struct BreakInPool {
    inner: OrderedPool<BreakInPoolEntry>,
}

impl BreakInPool {
    pub fn sync<'a>(
        &'a self,
        token: &mut Option<BreakInPoolToken<'a>>,
        entry: Option<BreakInPoolEntry>,
    ) {
        self.inner.sync(token, entry)
    }

    pub fn sweep_expired(&self, ttl: Duration) {
        self.inner.sweep_expired(ttl);
    }

    pub fn get(&self, key: &BreakInPoolKey) -> Option<BreakInPoolEntry> {
        self.inner.get(key)
    }

    pub fn matches(&self, query: &BreakInPoolQuery) -> Vec<BreakInPoolEntry> {
        self.inner
            .matches_bounded(query.limit as usize, Order::OldestFirst, |e| {
                query.matches(e)
            })
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
    pub updated_at: tokio::time::Instant,
}

impl Keyed for BreakInPoolEntry {
    type Key = BreakInPoolKey;

    fn key(&self) -> BreakInPoolKey {
        BreakInPoolKey(self.player_id)
    }

    fn updated_at(&self) -> tokio::time::Instant {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = tokio::time::Instant::now();
    }
}

#[derive(Clone, Debug)]
pub struct BreakInPoolQuery {
    pub player_id: i32,
    pub character_level: u32,
    pub weapon_level: u32,
    pub play_region: u32,
    pub limit: u32,
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
        let lower = invader.saturating_sub(invader / 10);

        let upper = if invader >= 301 {
            713
        } else {
            invader + (invader as f32 * 0.1).ceil() as u32 + 20
        };

        (lower..=upper).contains(&host)
    }

    fn check_weapon_level(host: u32, invader: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(invader) {
            entry.regular_range.contains(&host)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BreakInPoolKey(pub i32);

pub type BreakInPoolToken<'a> = OrderedPoolToken<'a, BreakInPoolEntry>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BreakInAttemptKey {
    pub target_id: BreakInPoolKey,
    pub invader_id: i32,
}

#[derive(Clone)]
pub struct BreakInAttempt {
    /// invader channel sender for push notifs
    pub invader_tx: Sender<Vec<u8>>,
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
            updated_at: tokio::time::Instant::now(),
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            limit: 10,
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
            updated_at: tokio::time::Instant::now(),
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 400,
            weapon_level: 1,
            play_region: 0,
            limit: 10,
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
            updated_at: tokio::time::Instant::now(),
        };

        let invader = BreakInPoolQuery {
            player_id: 2,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            limit: 10,
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
            updated_at: tokio::time::Instant::now(),
        };

        let invader = BreakInPoolQuery {
            player_id: 1,
            character_level: 1,
            weapon_level: 1,
            play_region: 0,
            limit: 10,
        };

        assert!(!invader.matches(&host));
    }
}
