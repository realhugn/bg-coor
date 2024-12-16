use bg_coor::core::{Task, TaskStatus, TaskError};
use chrono::Utc;

#[test]
fn test_task_creation() {
    let name = "test_task".to_string();
    let payload = vec![1, 2, 3];
    let task = Task::new(name.clone(), payload.clone(), 3);

    assert_eq!(task.name(), name);
    assert_eq!(task.payload(), payload);
    assert_eq!(*task.status(), TaskStatus::Pending);
    assert_eq!(task.retries(), 0);
    assert_eq!(task.max_retries(), 3);
    assert!(task.created_at() <= Utc::now());
}

#[test]
fn test_task_status_transitions() {
    let mut task = Task::new("test".to_string(), vec![], 3);
    
    assert_eq!(task.status(), &TaskStatus::Pending);
    
    task.set_status(TaskStatus::Running);
    assert_eq!(task.status(), &TaskStatus::Running);
    
    task.set_status(TaskStatus::Completed);
    assert_eq!(task.status(), &TaskStatus::Completed);
    
    task.set_status(TaskStatus::Failed("error".to_string()));
    match task.status() {
        TaskStatus::Failed(msg) => assert_eq!(msg, "error"),
        _ => panic!("Expected Failed status"),
    }
}

#[test]
fn test_task_serialization() {
    let task = Task::new("test".to_string(), vec![1, 2, 3], 3);
    let serialized = serde_json::to_string(&task).unwrap();
    let deserialized: Task = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(task.id(), deserialized.id());
    assert_eq!(task.status(), deserialized.status());
}

#[test]
fn test_task_display() {
    let task = Task::new("test_task".to_string(), vec![], 3);
    let display = format!("{}", task);
    assert!(display.contains("test_task"));
    assert!(display.contains(&task.id().to_string()));
}

#[test]
fn test_task_error_conversions() {
    let err = TaskError::NotFound("test".to_string());
    assert_eq!(err.to_string(), "Task not found: test");

    let err = TaskError::MaxRetriesExceeded;
    assert_eq!(err.to_string(), "Maximum retries exceeded");
}