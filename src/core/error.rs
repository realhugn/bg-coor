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

    #[error("Task validation failed: {0}")]
    ValidationError(String),

    #[error("Task handler not found: {0}")]
    HandlerNotFound(String),

    #[error("Invalid task signature")]
    InvalidSignature,

    #[error("Task registry is locked: {0}")]
    RegistryLocked(String),
}