use std::sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;

pub mod key;
pub mod pool;
pub mod sign;
pub mod breakin;
pub mod matching;

use self::pool::Pool;
use self::sign::SignPoolEntry;
use self::breakin::InvasionPoolEntry;

static SIGN_POOL: OnceLock<RwLock<Pool<SignPoolEntry>>> = OnceLock::new();
static INVASION_POOL: OnceLock<RwLock<Pool<InvasionPoolEntry>>> = OnceLock::new();

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Could not find pool entry")]
    NotFound,

    #[error("Could not insert pool entry")]
    Insert,

    #[error("Could not obtain pool lock")]
    Lock,

    #[error("Could not initialize pool")]
    Initialize,

    #[error("Requested pool that wasn't initialized")]
    NotInitialized,
}

pub fn init_pools() -> Result<(), PoolError> {
    SIGN_POOL.set(RwLock::new(Pool::default()))
        .map_err(|_| PoolError::Initialize)?;

    INVASION_POOL.set(RwLock::new(Pool::default()))
        .map_err(|_| PoolError::Initialize)?;

    Ok(())
}

pub fn sign_pool() -> RwLockReadGuard<'static, Pool<SignPoolEntry>> {
    safe_read_lock(SIGN_POOL.get().unwrap())
}

pub fn sign_pool_mut() -> RwLockWriteGuard<'static, Pool<SignPoolEntry>> {
    safe_write_lock(SIGN_POOL.get().unwrap())
}

pub fn invasion_pool() -> RwLockReadGuard<'static, Pool<InvasionPoolEntry>> {
    safe_read_lock(INVASION_POOL.get().unwrap())
}

pub fn invasion_pool_mut() -> RwLockWriteGuard<'static, Pool<InvasionPoolEntry>> {
    safe_write_lock(INVASION_POOL.get().unwrap())
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
