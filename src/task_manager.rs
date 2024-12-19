use std::sync::Arc;

use uuid::Uuid;

use crate::broker::memory::MemoryBroker;
use crate::broker::traits::Broker;
use crate::core::{Task, TaskError, TaskSignature};
use crate::storage::{MemoryStorage, Storage};
use crate::worker::pool::WorkerPool;
use crate::worker::registry::{TaskHandler, TaskRegistry};

pub struct TaskManagerBuilder {
    broker: Option<Arc<dyn Broker>>,
    storage: Option<Arc<dyn Storage>>,
    registry: Option<Arc<TaskRegistry>>,
    concurrency: usize,
}

impl TaskManagerBuilder {
    pub fn new(concurrency: usize) -> Self {
        Self {
            broker: None,
            storage: None,
            registry: None,
            concurrency,
        }
    }

    pub fn with_broker<B: Broker + 'static>(mut self, broker: B) -> Self {
        self.broker = Some(Arc::new(broker));
        self
    }

    pub fn with_storage<S: Storage + 'static>(mut self, storage: S) -> Self {
        self.storage = Some(Arc::new(storage));
        self
    }

    pub fn with_registry(mut self, registry: TaskRegistry) -> Self {
        self.registry = Some(Arc::new(registry));
        self
    }

    pub fn build(self) -> TaskManager {
        let broker = self.broker.unwrap_or_else(|| Arc::new(MemoryBroker::new()));
        let storage = self.storage.unwrap_or_else(|| Arc::new(MemoryStorage::new()));
        let registry = self.registry.unwrap_or_else(|| Arc::new(TaskRegistry::new()));
        
        let pool = WorkerPool::new(
            broker.clone(),
            storage.clone(),
            registry.clone(),
            self.concurrency,
        );

        TaskManager {
            broker,
            storage, 
            registry,
            pool,
        }
    }
}


pub struct TaskManager {
    broker: Arc<dyn Broker>,
    storage: Arc<dyn Storage>,
    registry: Arc<TaskRegistry>,
    pool: WorkerPool,
}

impl TaskManager {
    pub fn builder(concurrency: usize) -> TaskManagerBuilder {
        TaskManagerBuilder::new(concurrency)
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
