use std::sync::Arc;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

use crate::core::{Task, TaskStatus, TaskError};
use crate::broker::traits::Broker;

pub struct Executor {
    broker:  Arc<dyn Broker>
}

impl Executor {
    pub fn new(broker: Arc<dyn Broker>) -> Self {
        Executor {
            broker
        }
    }

    pub async fn execute_task(&self, mut task: Task) -> Result<(), TaskError> {
        task.set_status(TaskStatus::Running);
        self.broker.update_task(&task).await?;

        match self.process_task(&task).await {
            Ok(_) => {
                task.set_status(TaskStatus::Completed);
                self.broker.update_task(&task).await?;
                Ok(())
            }
            Err(e) => {
                error!("Task execution failed: {}", e);
                task.set_status(TaskStatus::Failed(e.to_string()));
                self.broker.update_task(&task).await?;
                Err(e)
            }
        }
    }

    async fn process_task(&self, task: &Task) -> Result<(), TaskError> {
        // Simulate task processing
        info!("Processing task: {}", task);
        sleep(Duration::from_secs(1)).await;        

        Ok(())
    }
}