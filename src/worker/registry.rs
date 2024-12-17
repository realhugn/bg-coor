use std::{collections::HashMap, sync::{RwLock, Arc}};

use async_trait::async_trait;
use crate::core::TaskError;

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

    pub fn register<H>(&self, name: &str, handler: H)
    where
        H: TaskHandler + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(name.to_string(), Arc::new(handler));
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn TaskHandler<>>> {
        let handlers = self.handlers.read().unwrap();
        handlers.get(name).map(Arc::clone)
    }
}