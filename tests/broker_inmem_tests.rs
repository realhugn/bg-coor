use bg_coor::broker::memory::MemoryBroker;
use bg_coor::broker::traits::Broker;
use bg_coor::core::Task;

#[tokio::test]
async fn test_memory_broker() {
    let broker = MemoryBroker::new();
    let task = Task::new("test_task".to_string(), vec![], 3);

    // Test push
    broker.push(&task).await.unwrap();

    // Test get_task
    let retrieved = broker.get_task(task.id()).await.unwrap().unwrap();
    assert_eq!(retrieved.id(), task.id());

    // Test pop
    let popped = broker.pop().await.unwrap().unwrap();
    assert_eq!(popped.id(), task.id());

    // Test empty pop
    let empty = broker.pop().await.unwrap();
    assert!(empty.is_none());
}
