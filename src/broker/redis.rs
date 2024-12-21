use super::traits::Broker;
use crate::core::{Task, TaskError};

use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use uuid::Uuid;

pub struct RedisBroker {
    client: Client,
    queue_key: String,
}

impl RedisBroker {
    pub fn new(redis_url: &str, queue_key: &str) -> Result<Self, redis::RedisError> {
        Ok(Self {
            client: Client::open(redis_url)?,
            queue_key: queue_key.to_string(),
        })
    }
}

#[async_trait]
impl Broker for RedisBroker {
    async fn push(&self, task: &Task) -> Result<(), TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let task_json = serde_json::to_string(task)?;

        let _: i64 = conn.lpush(&self.queue_key, &task_json).await?;

        let _: () = conn.set(task.id().to_string(), task_json).await?;

        Ok(())
    }

    async fn pop(&self) -> Result<Option<Task>, TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let task_json: Option<String> = conn.rpop(&self.queue_key, None).await?;

        if let Some(task_json) = task_json {
            let task: Task = serde_json::from_str(&task_json)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    async fn get_task(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let task_json: Option<String> = conn.get(id.to_string()).await?;

        if let Some(task_json) = task_json {
            let task: Task = serde_json::from_str(&task_json)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    async fn update_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let task_json = serde_json::to_string(task)?;

        let _: String = conn.set(task.id().to_string(), task_json).await?;
        Ok(())
    }
}
