use std::time::Duration;

use sqlx::{Pool, Postgres};

use super::attempt_tracker::AttemptTracker;
use breakin::{BreakInAttempt, BreakInAttemptKey, BreakInPool};
use quickmatch::MatchmakingService;
use sign::{SignPool, SummonAttempt, SummonAttemptKey};
use visit::{VisitAttemptKey, VisitorAttempt, VisitorPool};

use crate::{bans::BanService, notification::NotificationChannelPool, steam::SteamServer};

pub mod area;
pub mod breakin;
pub mod quickmatch;
pub mod sign;
pub mod visit;
pub mod weapon;

pub struct GameServices {
    pub database: Pool<Postgres>,
    pub steam: SteamServer,
    pub bans: BanService,
    pub quickmatch: MatchmakingService,
    pub pool_sign: SignPool,
    pub pool_breakin: BreakInPool,
    pub pool_visitor: VisitorPool,
    pub summon_attempts: AttemptTracker<SummonAttemptKey, SummonAttempt>,
    pub breakin_attempts: AttemptTracker<BreakInAttemptKey, BreakInAttempt>,
    pub visit_attempts: AttemptTracker<VisitAttemptKey, VisitorAttempt>,
    pub notifications: NotificationChannelPool,
}

impl GameServices {
    pub fn new(database: Pool<Postgres>) -> Result<GameServices, Box<dyn std::error::Error>> {
        Ok(GameServices {
            bans: BanService::new(database.clone()),
            database,
            steam: SteamServer::init()?,
            quickmatch: MatchmakingService::default(),
            pool_sign: SignPool::default(),
            pool_breakin: BreakInPool::default(),
            pool_visitor: VisitorPool::default(),
            summon_attempts: AttemptTracker::default(),
            breakin_attempts: AttemptTracker::default(),
            visit_attempts: AttemptTracker::default(),
            notifications: NotificationChannelPool::default(),
        })
    }

    pub fn sweep_expired(&self) {
        const QUICKMATCH_TTL: Duration = Duration::from_secs(70);
        const QUICKMATCH_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);
        const SIGN_TTL: Duration = Duration::from_secs(40);
        const SIGN_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);
        const BREAKIN_TTL: Duration = Duration::from_secs(20);
        const VISITOR_TTL: Duration = Duration::from_secs(20);
        const BREAKIN_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);
        const VISIT_ATTEMPT_CLEANUP_TIMEOUT: Duration = Duration::from_secs(30);

        self.quickmatch
            .sweep_expired(QUICKMATCH_TTL, QUICKMATCH_ATTEMPT_CLEANUP_TIMEOUT);
        self.pool_sign.sweep_expired(SIGN_TTL);
        self.summon_attempts
            .sweep_expired(SIGN_ATTEMPT_CLEANUP_TIMEOUT);
        self.pool_breakin.sweep_expired(BREAKIN_TTL);
        self.breakin_attempts
            .sweep_expired(BREAKIN_ATTEMPT_CLEANUP_TIMEOUT);
        self.pool_visitor.sweep_expired(VISITOR_TTL);
        self.visit_attempts
            .sweep_expired(VISIT_ATTEMPT_CLEANUP_TIMEOUT);
    }
}
