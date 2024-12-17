use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub(crate) result: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Cancelled,
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
            result: None,
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

    pub fn is_ready(&self) -> bool {
        self.status == TaskStatus::Pending
    }

    pub fn increment_retries(&mut self) {
        self.retries += 1;
    }

    pub fn get_result(&self) -> Option<&[u8]> {
        self.result.as_deref()
    }

    pub fn set_result(&mut self, result: Vec<u8>) {
        self.result = Some(result);
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Task({}, {})", self.id, self.name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskSignature {
    pub name: String,
    pub args: Vec<Value>,
    pub kwargs: HashMap<String, serde_json::Value>,
}

impl TaskSignature {
    pub fn new(name: String, args: Vec<Value>, kwargs: HashMap<String, serde_json::Value>) -> Self {
        TaskSignature { name, args, kwargs }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}