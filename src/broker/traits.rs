use async_trait::async_trait;
use crate::core::{Task, TaskError};

#[async_trait]
pub trait Broker: Send + Sync {
    async fn push(&self, task: &Task) -> Result<(), TaskError>;
    async fn pop(&self) -> Result<Option<Task>, TaskError>;
    async fn get_task(&self, id: uuid::Uuid) -> Result<Option<Task>, TaskError>;
    async fn update_task(&self, task: &Task) -> Result<(), TaskError>;
}