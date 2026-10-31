use std::{hash::Hash, time::Duration};

use indexmap::IndexMap;
use parking_lot::RwLock;
use thiserror::Error;
use tokio::time::Instant;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Entry not found")]
    NotFound,
}

pub trait Keyed {
    type Key: Clone + Hash + Eq;

    fn key(&self) -> Self::Key;
    fn updated_at(&self) -> Instant;
    fn touch(&mut self);
}

/// Iteration order over pool entries, by insertion time.
#[derive(Clone, Copy, Debug)]
pub enum Order {
    OldestFirst,
    NewestFirst,
}

pub struct OrderedPool<V: Keyed> {
    entries: RwLock<IndexMap<V::Key, V>>,
}

impl<V: Keyed> Default for OrderedPool<V> {
    fn default() -> Self {
        Self {
            entries: RwLock::new(IndexMap::new()),
        }
    }
}

impl<V: Keyed> OrderedPool<V> {
    pub fn insert(&self, value: V) -> OrderedPoolToken<'_, V> {
        let key = value.key();
        self.entries.write().insert(key.clone(), value);
        OrderedPoolToken { pool: self, key }
    }

    pub fn remove(&self, key: &V::Key) -> Result<(), PoolError> {
        self.entries
            .write()
            .shift_remove(key)
            .map(|_| ())
            .ok_or(PoolError::NotFound)
    }

    pub fn sync<'a>(&'a self, slot: &mut Option<OrderedPoolToken<'a, V>>, value: Option<V>) {
        match (slot.take(), value) {
            (_, None) => {}
            (Some(token), Some(value)) if token.key == value.key() => {
                self.entries.write().insert(token.key.clone(), value);
                *slot = Some(token);
            }
            (old, Some(value)) => {
                drop(old);
                *slot = Some(self.insert(value));
            }
        }
    }

    pub fn merge(&self, key: &V::Key, merger: impl FnOnce(&mut V)) -> Result<(), PoolError> {
        let mut entries = self.entries.write();
        let entry = entries.get_mut(key).ok_or(PoolError::NotFound)?;
        merger(entry);
        entry.touch();
        Ok(())
    }

    pub fn sweep_expired(&self, ttl: Duration) {
        let now = Instant::now();
        self.entries
            .write()
            .retain(|_, v| now.duration_since(v.updated_at()) < ttl);
    }
}

impl<V: Keyed + Clone> OrderedPool<V> {
    pub fn get(&self, key: &V::Key) -> Option<V> {
        self.entries.read().get(key).cloned()
    }

    pub fn matches_bounded(
        &self,
        limit: usize,
        order: Order,
        mut predicate: impl FnMut(&V) -> bool,
    ) -> Vec<V> {
        let entries = self.entries.read();
        let values: Box<dyn Iterator<Item = &V>> = match order {
            Order::OldestFirst => Box::new(entries.values()),
            Order::NewestFirst => Box::new(entries.values().rev()),
        };

        values
            .filter(|v| predicate(v))
            .take(limit)
            .cloned()
            .collect()
    }

    pub fn matches_all(&self, predicate: impl FnMut(&V) -> bool) -> Vec<V> {
        self.matches_bounded(usize::MAX, Order::OldestFirst, predicate)
    }
}

/// Removes its entry from the pool when dropped.
pub struct OrderedPoolToken<'a, V: Keyed> {
    pool: &'a OrderedPool<V>,
    key: V::Key,
}

impl<V: Keyed> OrderedPoolToken<'_, V> {
    pub fn key(&self) -> &V::Key {
        &self.key
    }
}

impl<V: Keyed> Drop for OrderedPoolToken<'_, V> {
    fn drop(&mut self) {
        let _ = self.pool.remove(&self.key);
    }
}
