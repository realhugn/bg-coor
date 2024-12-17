// src/storage/memory.rs
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::core::{Task, TaskError};
use super::traits::Storage;

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