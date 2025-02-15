use thiserror::Error;
use sqlx::{Pool, Postgres};

use breakin::BreakInPool;
use quickmatch::QuickMatchPool;
use sign::SignPool;
use visit::VisitorPool;

use crate::steam::SteamServer;

pub mod area;
pub mod breakin;
pub mod quickmatch;
pub mod sign;
pub mod visit;
pub mod weapon;

pub struct GameServices {
    pub database: Pool<Postgres>,
    pub steam: SteamServer,
    pub pool_sign: SignPool,
    pub pool_breakin: BreakInPool,
    pub pool_visitor: VisitorPool,
    pub pool_quickmatch: QuickMatchPool,
}

impl GameServices {
    pub fn new(database: Pool<Postgres>) -> Result<GameServices, Box<dyn std::error::Error>> {
        Ok(GameServices {
            database,
            steam: SteamServer::init()?,
            pool_sign: SignPool::default(),
            pool_breakin: BreakInPool::default(),
            pool_visitor: VisitorPool::default(),
            pool_quickmatch: QuickMatchPool::default(),
        })
    }
}

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Entry not found")]
    NotFound,
}
