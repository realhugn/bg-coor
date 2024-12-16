use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task execution failed: {0}")]
    ExecutionError(String),
    
    #[error("Task serialization failed: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Task not found: {0}")]
    NotFound(String),
    
    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,
}