use std::sync::Arc;

use uuid::Uuid;

use crate::broker::memory::MemoryBroker;
use crate::broker::traits::Broker;
use crate::core::{Task, TaskError, TaskSignature};
use crate::storage::{MemoryStorage, Storage};
use crate::worker::pool::WorkerPool;
use crate::worker::registry::{TaskHandler, TaskRegistry};

pub struct TaskManager {
    broker: Arc<dyn Broker>,
    storage: Arc<dyn Storage>,
    registry: Arc<TaskRegistry>,
    pool: WorkerPool,
}

impl TaskManager {
    pub fn new(concurrency: usize) -> Self {
        let broker = Arc::new(MemoryBroker::new());
        let storage = Arc::new(MemoryStorage::new());
        let registry = Arc::new(TaskRegistry::new());
        let pool = WorkerPool::new(
            broker.clone(),
            storage.clone(),
            registry.clone(),
            concurrency,
        );
        TaskManager {
            broker,
            storage,
            registry,
            pool,
        }
    }

    pub async fn start(&mut self) -> Result<(), TaskError> {
        self.pool.start().await
    }

    pub async fn shutdown(&mut self) -> Result<(), TaskError> {
        self.pool
            .shutdown()
            .await
            .map_err(|e| TaskError::ShutdownError(e.to_string()))
    }

    pub fn register_handler<H>(&self, name: &str, handler: H) -> Result<(), TaskError>
    where
        H: TaskHandler + 'static,
    {
        self.registry.register(name, handler)
    }

    pub async fn enqueue_task(
        &self,
        signature: TaskSignature,
        max_retries: u32,
    ) -> Result<Uuid, TaskError> {
        let payload = signature.to_bytes();

        let task = Task::new(signature.name.to_string(), payload, max_retries);
        self.broker.push(&task).await?;

        Ok(task.id)
    }

    pub async fn list_tasks(&self) -> Result<Vec<Task>, TaskError> {
        self.storage.list_tasks().await
    }

    pub async fn get_task(&self, id: uuid::Uuid) -> Result<Option<Task>, TaskError> {
        self.storage.load_task(id).await
    }
}
