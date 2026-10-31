use std::{hash::Hash, time::Duration};

use dashmap::DashMap;
use tokio::time::Instant;

struct Attempt<V> {
    inserted_at: Instant,
    value: V,
}

/// Short-lived pending attempts (summons, invasions, visits, joins) keyed by
/// the participants, swept once they outlive their TTL.
pub struct AttemptTracker<K: Eq + Hash, V> {
    entries: DashMap<K, Attempt<V>>,
}

impl<K: Eq + Hash, V> Default for AttemptTracker<K, V> {
    fn default() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> AttemptTracker<K, V> {
    pub fn insert(&self, key: K, value: V) {
        self.entries.insert(
            key,
            Attempt {
                inserted_at: Instant::now(),
                value,
            },
        );
    }

    pub fn remove(&self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|(_, attempt)| attempt.value)
    }

    pub fn sweep_expired(&self, ttl: Duration) {
        let now = Instant::now();
        self.entries
            .retain(|_, attempt| now.duration_since(attempt.inserted_at) < ttl);
    }
}
