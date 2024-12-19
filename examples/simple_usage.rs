use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use bg_coor::{core::TaskError, task_manager::TaskManager, worker::registry::TaskHandler};
use serde_json::{json, Value};
use tokio::time::sleep;

pub struct AddTask;

#[async_trait]
impl TaskHandler for AddTask {
    async fn handle(
        &self,
        args: Vec<Value>,
        _kwargs: HashMap<String, Value>,
    ) -> Result<Vec<u8>, TaskError> {
        // Parse arguments
        let a = args.get(0).and_then(|v| v.as_i64()).ok_or_else(|| {
            TaskError::InvalidArgument("First argument missing or invalid".into())
        })?;

        let b = args.get(1).and_then(|v| v.as_i64()).ok_or_else(|| {
            TaskError::InvalidArgument("Second argument missing or invalid".into())
        })?;

        // Simulate long running task
        println!("Starting addition of {} + {}", a, b);
        sleep(Duration::from_secs(3)).await;

        // Perform addition
        let result = a + b;
        println!("Result: {}", result);

        // Return result as bytes
        Ok(result.to_string().into_bytes())
    }
}

#[tokio::main]
async fn main() -> Result<(), TaskError> {
    let mut manager = TaskManager::builder(2).build();
    manager.register_handler("add", AddTask)?;

    manager.start().await?;

    // Create task signature
    let signature = bg_coor::core::TaskSignature {
        name: "add".to_string(),
        args: vec![json!(5), json!(3)],
        kwargs: HashMap::new(),
    };

    // Enqueue the task
    let task_id = manager.enqueue_task(signature, 3).await?;
    println!("Task ID: {}", task_id);

    loop {
        let task_result = manager.get_task(task_id).await?;
        match task_result {
            Some(task) => {
                if task.is_finished() {
                    let string_result = task
                        .get_result()
                        .map(|r| String::from_utf8_lossy(r).to_string())
                        .unwrap_or_else(|| "No result".to_string());
                    println!("Task result: {}", string_result);
                    break;
                }
            }
            None => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    manager.shutdown().await
}
