# bg-coor
A distributed background task processing system written in Rust, focused on reliability and ease of use.

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
use bg_coor::broker::memory::MemoryBroker;
use bg_coor::storage::MemoryStorage;
use bg_coor::worker::{pool::WorkerPool, registry::TaskRegistry};

#[tokio::main]
async fn main() {
    // Initialize components
    let broker = Arc::new(MemoryBroker::new());
    let storage = Arc::new(MemoryStorage::new());
    let registry = Arc::new(TaskRegistry::new());

    // Register task handlers
    registry.register("my_task", MyTaskHandler).unwrap();

    // Create and start worker pool
    let mut pool = WorkerPool::new(broker.clone(), storage, registry, 4);
    pool.start().await.unwrap();

    // Create and enqueue task
    let task = Task::new("my_task".to_string(), payload, 3);
    broker.push(&task).await.unwrap();

    // Shutdown gracefully
    pool.shutdown().await.unwrap();
}
```
## Roadmap
Future development plans:

<input disabled="" type="checkbox"> Persistent storage backends (Redis, PostgreSQL) </br>
<input disabled="" type="checkbox"> Distributed broker implementations</br>
<input disabled="" type="checkbox"> Task scheduling with cron expressions</br>
<input disabled="" type="checkbox"> Task dependencies and workflow support</br>
<input disabled="" type="checkbox"> Web interface for task monitoring</br>
<input disabled="" type="checkbox"> Metrics and monitoring</br>
<input disabled="" type="checkbox"> Task result serialization formats</br>
<input disabled="" type="checkbox"> Dead letter queue for failed tasks</br>
<input disabled="" type="checkbox"> Task prioritization</br>
<input disabled="" type="checkbox"> Rate limiting and backpressure</br>