# bg-coor
**bg-coor** is a background task coordination library for Rust

## Features

Current features include:

- Asynchronous task processing using Tokio
- In-memory task broker and storage implementations
- Configurable worker pools with concurrent task execution
- JSON-based task signatures for flexible payload handling
- Thread-safe task registry with dynamic handler registration

## Architecture

The system consists of several core components:

- **Task**: Core unit of work with unique ID, payload, and status tracking
- **Broker**: Handles task queue management and distribution
- **Storage**: Manages task persistence and state
- **Worker Pool**: Manages concurrent task execution
- **Task Registry**: Maps task names to their handlers
- **Executor**: Executes individual tasks with error handling and retries

## Usage

Basic example:

```rust
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
```
## Roadmap
Future development plans:
- [x] Implement a task manager wrapper to simplify lib usage.
- [ ] Persistent storage backends (Redis, PostgreSQL)
- [ ] Distributed broker implementations
- [ ] Task scheduling with cron expressions
- [ ] Task dependencies and workflow support
- [ ] Web interface for task monitoring
- [ ] Metrics and monitoring
- [ ] Task result serialization formats
- [ ] Dead letter queue for failed tasks
- [ ] Task prioritization
- [ ] Rate limiting and backpressure

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to update tests as appropriate and follow the existing coding style.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
