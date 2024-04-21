use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};

use super::key::PoolKey;
use super::PoolError;

pub struct MatchResult<TEntry>(pub PoolKey, pub TEntry);

#[derive(Debug)]
pub struct Pool<TEntry: Clone> {
    counter: AtomicI32,
    entries: HashMap<PoolKey, TEntry>,
}

impl<TEntry: Clone> Default for Pool<TEntry> {
    fn default() -> Self {
        Self {
            counter: AtomicI32::default(),
            entries: Default::default(),
        }
    }
}

// TODO: think of some lock-free structure instead
impl<TEntry: Clone> Pool<TEntry> {
    pub fn match_entries<
        TQuery,
        TMatcher: PoolEntryMatcher::<TEntry, TQuery>
    >(&self, query: &TQuery) -> Vec<MatchResult<TEntry>> {

        self.entries.iter()
            .filter_map(|(id, e)| {
                if TMatcher::matches(e, query) {
                    Some(MatchResult(id.clone(), e.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn has(&self, key: &PoolKey) -> bool {
        self.entries.contains_key(key)
    }

    pub fn insert(
        &mut self,
        topic: i32,
        entry: TEntry,
    ) -> Result<PoolKey, PoolError> {
        let identifier = self.counter.fetch_add(1, Ordering::Relaxed);
        let key = PoolKey(identifier, topic);

        self.entries.insert(key.clone(), entry);

        Ok(key)
    }

    pub fn remove(&mut self, key: &PoolKey) -> Result<(), PoolError> {
        self.entries.remove(key)
            .ok_or(PoolError::NotFound)?;

        Ok(())
    }
}

pub trait PoolEntryMatcher<TEntry, TQuery> {
    fn matches(entry: &TEntry, query: &TQuery) -> bool;
}

#[cfg(test)]
mod test {
    use super::Pool;
    use super::PoolEntryMatcher;

    #[derive(Clone)]
    pub struct MockEntry;
    pub struct MockQuery;

    pub struct MockMatcherTrue;
    impl PoolEntryMatcher<MockEntry, MockQuery> for MockMatcherTrue {
        fn matches(_: &MockEntry, _: &MockQuery) -> bool {
            true
        }
    }

    pub struct MockMatcherFalse;
    impl PoolEntryMatcher<MockEntry, MockQuery> for MockMatcherFalse {
        fn matches(_: &MockEntry, _: &MockQuery) -> bool {
            false
        }
    }

    #[test]
    fn matches_true() {
        let mut pool = Pool::<MockEntry>::default();
        pool.insert(1, MockEntry {}).unwrap();

        let matches = pool.match_entries::<_, MockMatcherTrue>(&MockQuery {});
        assert!(matches.len() == 1);
    }

    #[test]
    fn doesnt_match_false() {
        let mut pool = Pool::<MockEntry>::default();
        pool.insert(1, MockEntry {}).unwrap();

        let matches = pool.match_entries::<_, MockMatcherFalse>(&MockQuery {});
        assert!(matches.len() == 0);
    }

    #[test]
    fn remove_works() {
        let mut pool = Pool::<MockEntry>::default();
        let key = pool.insert(1, MockEntry {}).unwrap();
        pool.remove(&key).unwrap();

        let matches = pool.match_entries::<_, MockMatcherTrue>(&MockQuery {});
        assert!(matches.len() == 0);
    }
}
