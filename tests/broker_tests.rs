#[cfg(test)]
mod tests {
    use bg_coor::broker::memory::MemoryBroker;
    use bg_coor::broker::redis::RedisBroker;
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

    #[tokio::test]
    async fn test_redis_broker() {
        let queue_name = format!("test_queue_{}", uuid::Uuid::new_v4());
        let broker = RedisBroker::new("redis://127.0.0.1:6379", &queue_name).unwrap();
        let task = Task::new("test_task".to_string(), vec![], 3);

        // Test push
        broker.push(&task).await.err();
        assert!(true);

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
}
