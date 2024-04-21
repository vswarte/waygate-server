use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

pub mod key;
pub mod pool;
pub mod sign;
pub mod breakin;
pub mod matching;

use self::pool::Pool;
use self::sign::SignPoolEntry;
use self::breakin::BreakinPoolEntry;

static SIGN_POOL: OnceLock<RwLock<Pool<SignPoolEntry>>> = OnceLock::new();
static BREAKIN_POOL: OnceLock<RwLock<Pool<BreakinPoolEntry>>> = OnceLock::new();

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Could not find pool entry")]
    NotFound,

    #[error("Could not initialize pool")]
    Initialize,
}

pub fn init_pools() -> Result<(), PoolError> {
    SIGN_POOL.set(RwLock::new(Pool::default()))
        .map_err(|_| PoolError::Initialize)?;

    BREAKIN_POOL.set(RwLock::new(Pool::default()))
        .map_err(|_| PoolError::Initialize)?;

    log::info!("Initialized matching pools");

    Ok(())
}

pub fn sign_pool() -> RwLockReadGuard<'static, Pool<SignPoolEntry>> {
    safe_read_lock(SIGN_POOL.get().unwrap())
}

pub fn sign_pool_mut() -> RwLockWriteGuard<'static, Pool<SignPoolEntry>> {
    safe_write_lock(SIGN_POOL.get().unwrap())
}

fn safe_read_lock<TSubject>(subject: &'static RwLock<TSubject>) -> std::sync::RwLockReadGuard<'_, TSubject> {
    match subject.read() {
        Ok(g) => g,
        Err(poisoned) => {
            log::warn!("Pool recovering from mutex poisoning");
            poisoned.into_inner()
        },
    }
}

fn safe_write_lock<TSubject>(subject: &'static RwLock<TSubject>) -> std::sync::RwLockWriteGuard<'_, TSubject> {
    match subject.write() {
        Ok(g) => g,
        Err(poisoned) => {
            log::warn!("Pool recovering from mutex poisoning");
            poisoned.into_inner()
        },
    }
}
