use std::{
    collections::HashMap,
    sync::{Arc, RwLock, TryLockError},
};

use crate::core::TaskError;
use async_trait::async_trait;

#[async_trait]
pub trait TaskHandler: Send + Sync {
    async fn handle(
        &self,
        args: Vec<serde_json::Value>,
        kwargs: HashMap<String, serde_json::Value>,
    ) -> Result<Vec<u8>, TaskError>;
}

pub struct TaskRegistry {
    handlers: RwLock<HashMap<String, Arc<dyn TaskHandler>>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<H>(&self, name: &str, handler: H) -> Result<(), TaskError>
    where
        H: TaskHandler + 'static,
    {
        match self.handlers.try_write() {
            Ok(mut handlers) => {
                handlers.insert(name.to_string(), Arc::new(handler));
                Ok(())
            }
            Err(TryLockError::WouldBlock) => Err(TaskError::RegistryLocked(
                "Failed to acquire write lock".into(),
            )),
            Err(TryLockError::Poisoned(_)) => Err(TaskError::RegistryLocked(
                "Registry lock is poisoned".into(),
            )),
        }
    }

    pub fn get(&self, name: &str) -> Result<Option<Arc<dyn TaskHandler>>, TaskError> {
        match self.handlers.try_read() {
            Ok(handlers) => Ok(handlers.get(name).map(Arc::clone)),
            Err(TryLockError::WouldBlock) => Err(TaskError::RegistryLocked(
                "Failed to acquire read lock".into(),
            )),
            Err(TryLockError::Poisoned(_)) => Err(TaskError::RegistryLocked(
                "Registry lock is poisoned".into(),
            )),
        }
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}