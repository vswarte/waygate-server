use std::{cell::RefCell, collections::HashMap, future::Future};

use serde::{ser::SerializeMap, Serialize, Serializer};
use tokio::task::futures::TaskLocalFuture;

tokio::task_local! {
    /// A task-local context for logging.
    static LOG_CONTEXT: LogContext;
}

#[derive(Clone)]
pub struct LogContext {
    context: RefCell<HashMap<String, String>>,
}

impl LogContext {
    fn new() -> Self {
        Self {
            context: RefCell::new(HashMap::new()),
        }
    }

    pub fn with<F>(f: F) -> TaskLocalFuture<LogContext, F>
    where
        F: Future,
    {
        LOG_CONTEXT.scope(LogContext::new(), f)
    }

    pub fn insert(key: impl Into<String>, value: impl Into<String>) {
        LOG_CONTEXT.with(|ctx| {
            ctx.context.borrow_mut().insert(key.into(), value.into());
        });
    }

    #[allow(dead_code)]
    pub fn get(key: &str) -> Option<String> {
        LOG_CONTEXT.with(|ctx| ctx.context.borrow().get(key).cloned())
    }

    #[allow(dead_code)]
    pub fn iter<F, R>(f: F) -> R
    where
        F: FnOnce(&HashMap<String, String>) -> R,
    {
        LOG_CONTEXT.with(|ctx| f(&ctx.context.borrow()))
    }

    pub fn current() -> LogContext {
        LOG_CONTEXT.with(|ctx| (*ctx).clone())
    }
}

impl Serialize for LogContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.context.borrow().len()))?;
        for (key, value) in self.context.borrow().iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
