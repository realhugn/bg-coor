use async_trait::async_trait;
use redis::Client;
use uuid::Uuid;

use crate::core::{Task, TaskError};

use super::Storage;

pub struct RedisStorage {
    client: Client,
    prefix: String,
}

impl RedisStorage {
    pub fn new(redis_url: &str, prefix: &str) -> Result<Self, redis::RedisError> {
        Ok(Self {
            client: Client::open(redis_url)?,
            prefix: prefix.to_string(),
        })
    }
}

#[async_trait]
impl Storage for RedisStorage {
    async fn store_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", self.prefix, task.id);
        let _: String = redis::cmd("SET")
            .arg(&key)
            .arg(serde_json::to_string(task)?)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn load_task(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", self.prefix, id);
        let task_json: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut conn).await?;
        if let Some(task_json) = task_json {
            let task: Task = serde_json::from_str(&task_json)?;
            Ok(Some(task))
        } else {
            Ok(None)
        }
    }

    async fn update_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", self.prefix, task.id);
        let _: String = redis::cmd("SET")
            .arg(&key)
            .arg(serde_json::to_string(task)?)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    async fn delete_task(&self, id: Uuid) -> Result<(), TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}:{}", self.prefix, id);
        let _: i64 = redis::cmd("DEL").arg(&key).query_async(&mut conn).await?;
        Ok(())
    }

    async fn list_tasks(&self) -> Result<Vec<Task>, TaskError> {
        let mut conn = self.client.get_async_connection().await?;
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg(format!("{}:*", self.prefix))
            .query_async(&mut conn)
            .await?;
        let mut tasks = Vec::new();
        for key in keys {
            let task_json: String = redis::cmd("GET").arg(&key).query_async(&mut conn).await?;
            let task: Task = serde_json::from_str(&task_json)?;
            tasks.push(task);
        }
        Ok(tasks)
    }
}
