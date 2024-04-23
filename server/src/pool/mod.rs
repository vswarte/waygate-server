use std::sync::{OnceLock, RwLock};
use thiserror::Error;

pub mod key;
pub mod sign;
pub mod breakin;
pub mod matching;
pub mod quickmatch;

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use key::PoolKey;
use self::quickmatch::QuickmatchPoolEntry;
use self::sign::SignPoolEntry;
use self::breakin::BreakInPoolEntry;

static SIGN_POOL: OnceLock<Pool<SignPoolEntry>> = OnceLock::new();
static BREAKIN_POOL: OnceLock<Pool<BreakInPoolEntry>> = OnceLock::new();
static QUICKMATCH_POOL: OnceLock<Pool<QuickmatchPoolEntry>> = OnceLock::new();

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Could not initialize pool")]
    Initialize,

    #[error("Entry not found")]
    NotFound,

    #[error("Pool was not initialized")]
    Uninitialized,
}

pub fn init_pools() -> Result<(), PoolError> {
    SIGN_POOL.set(Pool::default())
        .map_err(|_| PoolError::Initialize)?;

    BREAKIN_POOL.set(Pool::default())
        .map_err(|_| PoolError::Initialize)?;

    QUICKMATCH_POOL.set(Pool::default())
        .map_err(|_| PoolError::Initialize)?;

    log::info!("Initialized matching pools");

    Ok(())
}

pub fn sign_pool() -> Result<&'static Pool<SignPoolEntry>, PoolError> {
    SIGN_POOL.get().ok_or(PoolError::Uninitialized)
}

pub fn breakin() -> Result<&'static Pool<BreakInPoolEntry>, PoolError> {
    BREAKIN_POOL.get().ok_or(PoolError::Uninitialized)
}

pub fn quickmatch_pool() -> Result<&'static Pool<QuickmatchPoolEntry>, PoolError> {
    QUICKMATCH_POOL.get().ok_or(PoolError::Uninitialized)
}

pub struct MatchResult<TEntry>(pub PoolKey, pub TEntry);

#[derive(Debug)]
pub struct Pool<TEntry: Clone> {
    counter: AtomicI32,
    entries: RwLock<HashMap<PoolKey, TEntry>>,
}

impl<TEntry: Clone> Default for Pool<TEntry> {
    fn default() -> Self {
        Self {
            counter: AtomicI32::default(),
            entries: Default::default(),
        }
    }
}

impl<TEntry: Clone> Pool<TEntry> {
    pub fn match_entries<TQuery: PoolQuery<TEntry>>(
        &self,
        query: &TQuery,
    ) -> Vec<MatchResult<TEntry>> {
        self.lock_read()
            .iter()
            .filter_map(|(id, e)| {
                if query.matches(e) {
                    Some(MatchResult(id.clone(), e.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn has(&self, key: &PoolKey) -> bool {
        self.lock_read().contains_key(key)
    }

    pub fn insert(
        &self,
        topic: i32,
        entry: TEntry,
    ) -> Result<PoolKey, PoolError> {
        log::info!("Adding entry {topic:?} to pool");
        let identifier = self.counter.fetch_add(1, Ordering::Relaxed);
        let key = PoolKey(identifier, topic);

        self.lock_write()
            .insert(key.clone(), entry);

        Ok(key)
    }

    pub fn remove(
        &self,
        key: &PoolKey,
    ) -> Result<(), PoolError> {
        log::info!("Remove entry {key:?} from pool");
        self.lock_write()
            .remove(key)
            .ok_or(PoolError::NotFound)?;

        Ok(())
    }

    fn lock_read(&self) -> RwLockReadGuard<'_, HashMap<PoolKey, TEntry>> {
        self.entries.read()
            .unwrap_or_else(|p| {
                log::warn!("Pool recovering from mutex poisoning");
                self.entries.clear_poison();
                p.into_inner()
            })
    }

    fn lock_write(&self) -> RwLockWriteGuard<'_, HashMap<PoolKey, TEntry>> {
        self.entries.write()
            .unwrap_or_else(|p| {
                log::warn!("Pool recovering from mutex poisoning");
                self.entries.clear_poison();
                p.into_inner()
            })
    }
}

pub trait PoolQuery<TEntry> {
    fn matches(&self, entry: &TEntry) -> bool;
}

#[cfg(test)]
mod test {
    use super::Pool;
    use super::PoolQuery;

    #[derive(Clone)]
    pub struct MockEntry;

    pub struct MockQueryTrue;
    impl PoolQuery<MockEntry> for MockQueryTrue {
        fn matches(&self, _: &MockEntry) -> bool {
            true
        }
    }

    pub struct MockQueryFalse;
    impl PoolQuery<MockEntry> for MockQueryFalse {
        fn matches(&self, _: &MockEntry) -> bool {
            false
        }
    }

    #[test]
    fn matches_true() {
        let pool = Pool::<MockEntry>::default();
        pool.insert(1, MockEntry {}).unwrap();

        let matches = pool.match_entries(&MockQueryTrue {});
        assert!(matches.len() == 1);
    }

    #[test]
    fn doesnt_match_false() {
        let pool = Pool::<MockEntry>::default();
        pool.insert(1, MockEntry {}).unwrap();

        let matches = pool.match_entries(&MockQueryFalse {});
        assert!(matches.is_empty());
    }

    #[test]
    fn remove_works() {
        let pool = Pool::<MockEntry>::default();
        let key = pool.insert(1, MockEntry {}).unwrap();
        pool.remove(&key).unwrap();

        let matches = pool.match_entries(&MockQueryTrue {});
        assert!(matches.is_empty());
    }
}
