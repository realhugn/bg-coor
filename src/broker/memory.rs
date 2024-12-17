use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::core::{Task, TaskError};
use super::traits::Broker;

pub struct MemoryBroker {
    tasks: Mutex<HashMap<Uuid, Task>>,
    queue: Mutex<Vec<Uuid>>
}

impl MemoryBroker {
    pub fn new() -> Self {
        MemoryBroker {
            tasks: Mutex::new(HashMap::new()),
            queue: Mutex::new(Vec::new())
        }
    }
}

#[async_trait]
impl Broker for MemoryBroker {
    async fn push(&self, task: &Task) -> Result<(), TaskError> {
        let id = task.id();
        let mut tasks = self.tasks.lock().await;
        let mut queue = self.queue.lock().await;

        tasks.insert(id, task.clone());
        queue.push(id);
        Ok(())
    }

    async fn pop(&self) -> Result<Option<Task>, TaskError> {
        let mut queue = self.queue.lock().await;
        if let Some(id) = queue.pop() {
            let tasks = self.tasks.lock().await;
            return Ok(tasks.get(&id).cloned());
        }
        Ok(None)
    }

    async fn get_task(&self, id: Uuid) -> Result<Option<Task>, TaskError> {
        let tasks = self.tasks.lock().await;
        Ok(tasks.get(&id).cloned())
    }

    async fn update_task(&self, task: &Task) -> Result<(), TaskError> {
        let mut tasks = self.tasks.lock().await;
        tasks.insert(task.id(), task.clone());
        Ok(())
    }
}

