use async_trait::async_trait;
use std::sync::Arc;

use crate::broker::traits::Broker;
use crate::core::{Task, TaskError, TaskSignature, TaskStatus};
use crate::storage::Storage;

use super::registry::{TaskHandler, TaskRegistry};

#[async_trait]
pub trait Middleware: Send + Sync {
    async fn before_execution(&self, task: &Task) -> Result<(), TaskError>;
    async fn after_execution(&self, task: &Task) -> Result<(), TaskError>;
}

pub struct Executor {
    broker: Arc<dyn Broker>,
    storage: Arc<dyn Storage>,
    registry: Arc<TaskRegistry>,
    middlewares: Vec<Box<dyn Middleware>>,
}

impl Executor {
    pub fn new(
        broker: Arc<dyn Broker>,
        storage: Arc<dyn Storage>,
        registry: Arc<TaskRegistry>,
    ) -> Self {
        Executor {
            broker,
            storage,
            registry,
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware<M: Middleware + 'static>(&mut self, middleware: M) {
        self.middlewares.push(Box::new(middleware));
    }

    pub async fn execute_task(&self, mut task: Task) -> Result<(), TaskError> {
        task.set_status(TaskStatus::Running);
        self.storage.update_task(&task).await?;
        let handler = match self.registry.get(&task.name())? {
            Some(h) => h,
            None => return Err(TaskError::HandlerNotFound(task.name().to_string())),
        };

        let result = self.process_task(&task, handler.as_ref()).await;
        match result {
            Ok(rs) => {
                task.set_status(TaskStatus::Completed);
                task.set_result(rs);
                self.storage.update_task(&task).await?;
                Ok(())
            }
            Err(e) => {
                if task.retries() < task.max_retries() {
                    task.increment_retries();
                    task.set_status(TaskStatus::Pending);
                    self.broker.push(&task).await?;
                    Ok(())
                } else {
                    task.set_status(TaskStatus::Failed(e.to_string()));
                    self.storage.update_task(&task).await?;
                    Err(e)
                }
            }
        }
    }

    async fn process_task(
        &self,
        task: &Task,
        handler: &dyn TaskHandler,
    ) -> Result<Vec<u8>, TaskError> {
        let signature: TaskSignature = serde_json::from_slice(&task.payload())?;

        if signature.name != task.name() {
            return Err(TaskError::InvalidSignature);
        }

        handler.handle(signature.args, signature.kwargs).await
    }
}
