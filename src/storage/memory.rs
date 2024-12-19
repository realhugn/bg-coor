// src/storage/memory.rs
use super::traits::Storage;
use crate::core::{Task, TaskError};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct MemoryStorage {
    tasks: RwLock<HashMap<Uuid, Task>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            tasks: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Storage for MemoryStorage {
    async fn store_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id(), task.clone());
        Ok(())
    }

    async fn load_task(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        let tasks = self.tasks.read().await;
        Ok(tasks.get(&id).cloned())
    }

    async fn update_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id(), task.clone());
        Ok(())
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), TaskError> {
        let mut tasks = self.tasks.write().await;
        tasks.remove(&id);
        Ok(())
    }

    async fn list_tasks(&self) -> Result<Vec<Task>, TaskError> {
        let tasks = self.tasks.read().await;
        Ok(tasks.values().cloned().collect())
    }
}
