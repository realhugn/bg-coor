use bg_coor::core::{Task, TaskStatus};
use bg_coor::storage::{MemoryStorage, Storage};

#[tokio::test]
async fn test_memory_storage() {
    let storage = MemoryStorage::new();
    let task = Task::new("test".to_string(), vec![1, 2, 3], 3);

    // Test store and load
    storage.store_task(&task).await.unwrap();
    let loaded = storage.load_task(task.id()).await.unwrap().unwrap();
    assert_eq!(loaded.id(), task.id());

    // Test update
    let mut updated = task.clone();
    updated.set_status(TaskStatus::Running);
    storage.update_task(&updated).await.unwrap();
    let loaded = storage.load_task(task.id()).await.unwrap().unwrap();
    assert_eq!(loaded.status(), &TaskStatus::Running);

    // Test list
    let tasks = storage.list_tasks().await.unwrap();
    assert_eq!(tasks.len(), 1);

    // Test delete
    storage.delete_task(task.id()).await.unwrap();
    assert!(storage.load_task(task.id()).await.unwrap().is_none());
}
