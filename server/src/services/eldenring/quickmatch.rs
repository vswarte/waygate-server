use message::eldenring::{QuickMatchSettings, QuickmatchTeamSlot};

use std::{sync::mpsc::Sender, time::Duration};

use tokio::time::Instant;

use crate::services::attempt_tracker::AttemptTracker;
use crate::services::eldenring::weapon;
use crate::services::ordered_pool::{Keyed, Order, OrderedPool, OrderedPoolToken, PoolError};

pub type QuickMatchPoolToken<'a> = OrderedPoolToken<'a, QuickMatchPoolEntry>;

type Pool = OrderedPool<QuickMatchPoolEntry>;

#[derive(Clone, Debug)]
pub struct QuickMatchPoolEntry {
    pub host_player_id: i32,
    pub host_external_id: String,
    pub character_level: u32,
    pub weapon_level: u32,
    pub arena_id: u32,
    pub password: String,
    pub quickmatch_settings: QuickMatchSettings,
    pub host_tx: Sender<Vec<u8>>,
    pub updated_at: Instant,
}

impl Keyed for QuickMatchPoolEntry {
    type Key = i32;

    fn key(&self) -> i32 {
        self.host_player_id
    }

    fn updated_at(&self) -> Instant {
        self.updated_at
    }

    fn touch(&mut self) {
        self.updated_at = Instant::now();
    }
}

#[derive(Debug)]
pub struct QuickMatchPoolQuery {
    pub player_id: i32,
    pub arenas: Vec<u32>,
    pub character_level: u32,
    pub weapon_level: u32,
    pub password: String,
    pub quickmatch_settings: QuickMatchSettings,
}

impl QuickMatchPoolQuery {
    fn matches(&self, entry: &QuickMatchPoolEntry) -> bool {
        if entry.host_player_id == self.player_id {
            return false;
        }

        if !self.arenas.contains(&entry.arena_id) {
            return false;
        }

        if !Self::settings_compatible(self.quickmatch_settings, entry.quickmatch_settings) {
            return false;
        }

        if !entry.quickmatch_settings.is_allies_password_mode()
            && (!entry.password.is_empty() || !self.password.is_empty())
        {
            return entry.password == self.password;
        }

        Self::check_character_level(self.character_level, entry.character_level)
            && Self::check_weapon_level(self.weapon_level, entry.weapon_level)
    }

    fn settings_compatible(searcher: QuickMatchSettings, host: QuickMatchSettings) -> bool {
        if searcher.match_size() != host.match_size() {
            return false;
        }
        if searcher.spirit_ashes_allowed() != host.spirit_ashes_allowed() {
            return false;
        }
        searcher.is_allies_password_mode() == host.is_allies_password_mode()
    }

    fn check_character_level(host: u32, joiner: u32) -> bool {
        let radius = (host as f32 * 0.1).ceil() as u32 + 10;
        let lower = host.saturating_sub(radius);
        let upper = if joiner >= 306 { 713 } else { host + radius };

        (lower..=upper).contains(&joiner)
    }

    fn check_weapon_level(host: u32, joiner: u32) -> bool {
        if let Some(entry) = weapon::get_level_table_entry(host) {
            entry.regular_range.contains(&joiner)
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QuickMatchJoinAttemptKey {
    pub host_player_id: i32,
    pub joining_player_id: i32,
}

#[derive(Clone)]
pub struct QuickMatchJoinAttempt {
    pub quickmatch_settings: QuickMatchSettings,
    pub team: QuickmatchTeamSlot,
    pub joining_player_tx: Sender<Vec<u8>>,
}

#[derive(Default)]
pub struct MatchmakingService {
    pool: Pool,
    pool_allied_pw: Pool,
    join_attempts: AttemptTracker<QuickMatchJoinAttemptKey, QuickMatchJoinAttempt>,
}

pub struct SearchResult {
    pub matches: Vec<QuickMatchPoolEntry>,
    pub target_team: QuickmatchTeamSlot,
    pub is_allied_password_opponent_search: bool,
}

impl MatchmakingService {
    fn pool(&self, allied_pw: bool) -> &Pool {
        if allied_pw {
            &self.pool_allied_pw
        } else {
            &self.pool
        }
    }

    fn candidates(
        pool: &Pool,
        query: &QuickMatchPoolQuery,
        limit: usize,
        require_password: bool,
    ) -> Vec<QuickMatchPoolEntry> {
        pool.matches_bounded(limit, Order::NewestFirst, |entry| {
            query.matches(entry)
                && (!require_password
                    || !entry.quickmatch_settings.is_allies_password_mode()
                    || query.password == entry.password)
        })
    }

    pub fn register(&self, entry: QuickMatchPoolEntry, allied_pw: bool) -> QuickMatchPoolToken<'_> {
        self.pool(allied_pw).insert(entry)
    }

    pub fn lobby_for_join(
        &self,
        host_player_id: i32,
        team: QuickmatchTeamSlot,
    ) -> Option<QuickMatchPoolEntry> {
        let ally_entry = self.pool.get(&host_player_id);
        let enemy_entry = self.pool_allied_pw.get(&host_player_id);
        if team == QuickmatchTeamSlot::Ally {
            ally_entry.or(enemy_entry)
        } else {
            enemy_entry.or(ally_entry)
        }
    }

    pub fn merge(
        &self,
        host_player_id: &i32,
        allied_pw: bool,
        merger: impl FnOnce(&mut QuickMatchPoolEntry),
    ) -> Result<(), PoolError> {
        self.pool(allied_pw).merge(host_player_id, merger)
    }

    pub fn record_join_attempt(
        &self,
        key: QuickMatchJoinAttemptKey,
        attempt: QuickMatchJoinAttempt,
    ) {
        self.join_attempts.insert(key, attempt);
    }

    pub fn take_join_attempt(
        &self,
        key: &QuickMatchJoinAttemptKey,
    ) -> Option<QuickMatchJoinAttempt> {
        self.join_attempts.remove(key)
    }

    pub async fn search(&self, query: &QuickMatchPoolQuery, limit: usize) -> SearchResult {
        let mut matches = Self::candidates(&self.pool, query, limit, true);
        let mut target_team = QuickmatchTeamSlot::Ally;
        let is_allied_password_opponent_search =
            query.quickmatch_settings.is_allies_password_mode();

        if matches.is_empty() && is_allied_password_opponent_search {
            matches = Self::candidates(&self.pool_allied_pw, query, limit, false);
            target_team = QuickmatchTeamSlot::Enemy;
        }

        SearchResult {
            matches,
            target_team,
            is_allied_password_opponent_search,
        }
    }

    pub fn sweep_expired(&self, pool_ttl: Duration, join_attempt_ttl: Duration) {
        self.pool.sweep_expired(pool_ttl);
        self.pool_allied_pw.sweep_expired(pool_ttl);
        self.join_attempts.sweep_expired(join_attempt_ttl);
    }
}
