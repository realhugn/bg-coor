// src/storage/traits.rs
use crate::core::{Task, TaskError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Storage: Send + Sync {
    async fn store_task(&self, task: &Task) -> Result<(), TaskError>;
    async fn load_task(&self, id: Uuid) -> Result<Option<Task>, TaskError>;
    async fn update_task(&self, task: &Task) -> Result<(), TaskError>;
    async fn delete_task(&self, id: Uuid) -> Result<(), TaskError>;
    async fn list_tasks(&self) -> Result<Vec<Task>, TaskError>;
}
