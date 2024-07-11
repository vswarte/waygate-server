use std::sync::{Arc, OnceLock, RwLock};
use fnrpc::shared::ObjectIdentifier;
use thiserror::Error;

pub mod key;
pub mod sign;
pub mod breakin;
pub mod matching;
pub mod quickmatch;

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use self::quickmatch::QuickmatchPoolEntry;
use self::sign::SignPoolEntry;
use self::breakin::BreakInPoolEntry;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Entry not found")]
    NotFound,
}

pub fn init_pools() -> Result<(), PoolError> {
    log::info!("Initialized matching pools");

    Ok(())
}

static SIGN_POOL: OnceLock<Pool<SignPoolEntry>> = OnceLock::new();
static BREAKIN_POOL: OnceLock<Pool<BreakInPoolEntry>> = OnceLock::new();
static QUICKMATCH_POOL: OnceLock<Pool<QuickmatchPoolEntry>> = OnceLock::new();

pub fn sign_pool() -> &'static Pool<SignPoolEntry> {
    SIGN_POOL.get_or_init(Default::default)
}

pub fn breakin_pool() -> &'static Pool<BreakInPoolEntry> {
    BREAKIN_POOL.get_or_init(Default::default)
}

pub fn quickmatch_pool() -> &'static Pool<QuickmatchPoolEntry> {
    QUICKMATCH_POOL.get_or_init(Default::default)
}

pub struct MatchResult<TEntry>(pub PoolKey, pub Arc<TEntry>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PoolKey(pub i32, pub i32);

impl From<&ObjectIdentifier> for PoolKey {
    fn from(value: &ObjectIdentifier) -> Self {
        Self(value.object_id, value.secondary_id)
    }
}

impl From<&PoolKey> for ObjectIdentifier {
    fn from(val: &PoolKey) -> Self {
        ObjectIdentifier {
            object_id: val.0,
            secondary_id: val.1,
        }
    }
}

#[derive(Debug)]
pub struct PoolKeyGuard<T: 'static>(&'static Pool<T>, pub PoolKey);

impl<T> Drop for PoolKeyGuard<T> {
    fn drop(&mut self) {
        log::info!("Dropped pool key guard");
        let _ = self.0.remove(&self.1);
    }
}

#[derive(Debug, Default)]
pub struct Pool<TEntry> {
    counter: AtomicI32,
    entries: RwLock<HashMap<PoolKey, Arc<TEntry>>>,
}

impl<TEntry> Pool<TEntry> {
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
        &'static self,
        topic: i32,
        entry: TEntry,
    ) -> Result<PoolKeyGuard<TEntry>, PoolError> {
        let identifier = self.counter.fetch_add(1, Ordering::Relaxed);
        let key = PoolKey(identifier, topic);

        log::info!("Adding entry {topic:?} to pool with key {identifier:?}");

        self.lock_write()
            .insert(key.clone(), Arc::new(entry));

        Ok(PoolKeyGuard(self, key))
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

    fn lock_read(&self) -> RwLockReadGuard<'_, HashMap<PoolKey, Arc<TEntry>>> {
        self.entries.read()
            .unwrap_or_else(|p| {
                log::warn!("Pool recovering from mutex poisoning");
                self.entries.clear_poison();
                p.into_inner()
            })
    }

    fn lock_write(&self) -> RwLockWriteGuard<'_, HashMap<PoolKey, Arc<TEntry>>> {
        self.entries.write()
            .unwrap_or_else(|p| {
                log::warn!("Pool recovering from mutex poisoning");
                self.entries.clear_poison();
                p.into_inner()
            })
    }

    pub fn by_topic_id(&self, topic: i32) -> Option<Arc<TEntry>> {
        self.lock_read()
            .iter()
            .find(|(k, _)| k.1 == topic)
            .map(|(_ ,v)| v.clone())
    }
}

pub trait PoolQuery<TEntry> {
    fn matches(&self, entry: &TEntry) -> bool;
}
