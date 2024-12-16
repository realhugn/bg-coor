use core::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) payload: Vec<u8>,
    pub(crate) status: TaskStatus,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) retries: u32,
    pub(crate) max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

impl Task {
    pub fn new(name: String, payload: Vec<u8>, max_retries: u32) -> Self {
        Task {
            id: Uuid::new_v4(),
            name,
            payload,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            retries: 0,
            max_retries,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn retries(&self) -> u32 {
        self.retries
    }

    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task({}, {})", self.id, self.name)
    }
}