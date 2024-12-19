use bg_coor::broker::memory::MemoryBroker;
use bg_coor::broker::traits::Broker;
use bg_coor::core::{Task, TaskError, TaskSignature, TaskStatus};
use bg_coor::storage::{MemoryStorage, Storage};
use bg_coor::worker::{executor::*, pool::*, registry::*};
use std::collections::HashMap;
use std::sync::Arc;

struct TestHandler;

#[async_trait::async_trait]
impl TaskHandler for TestHandler {
    async fn handle(
        &self,
        _args: Vec<serde_json::Value>,
        _kwargs: HashMap<String, serde_json::Value>,
    ) -> Result<Vec<u8>, TaskError> {
        Ok("It works!".as_bytes().to_vec())
    }
}

#[tokio::test]
async fn test_task_registry() {
    let registry = TaskRegistry::new();
    registry.register("test_task", TestHandler).unwrap();

    let handler = registry.get("test_task").unwrap();
    assert!(handler.is_some());

    let nonexistent = registry.get("nonexistent").unwrap();
    assert!(nonexistent.is_none());
}

#[tokio::test]
async fn test_executor() {
    let broker = Arc::new(MemoryBroker::new());
    let storage = Arc::new(MemoryStorage::new());
    let registry = Arc::new(TaskRegistry::new());

    registry.register("test_task", TestHandler).unwrap();

    let executor = Executor::new(broker, storage, registry);
    let task = Task::new("test_task".to_string(), vec![], 3);

    let result = executor.execute_task(task).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_worker_pool() {
    let broker = Arc::new(MemoryBroker::new());
    let storage = Arc::new(MemoryStorage::new());
    let registry = Arc::new(TaskRegistry::new());

    registry.register("test_task", TestHandler).unwrap();

    let mut pool = WorkerPool::new(broker.clone(), storage.clone(), registry.clone(), 1);
    pool.start().await.unwrap();

    let payload = TaskSignature::new("test_task".to_string(), vec![], HashMap::new());
    let task = Task::new("test_task".to_string(), payload.to_bytes(), 3);
    broker.push(&task).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    pool.shutdown().await.unwrap();

    let task_status = storage.load_task(task.id()).await.unwrap().unwrap();
    assert_eq!(task_status.status(), &TaskStatus::Completed);
}
