use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::storage::store_task;
use crate::task;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Incomplete,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    name: String,
    description: String,
    status: Status,
}

static TASK_ID_COUNTER: OnceLock<AtomicU32> = OnceLock::new();

impl Task {
    pub fn new(name: &str, description: &str) -> Self {
        // initialize the counter if it hasn't already been initialized
        let counter = TASK_ID_COUNTER.get_or_init(|| AtomicU32::new(1));

        // generate a unique id by incrementing the counter
        let id = counter.fetch_add(1, Ordering::SeqCst);

        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            status: Status::Incomplete,
        }
    }

    pub fn complete(id: usize) {
        let status = Status::Completed;

        println!("Task '' marked as completed.");
    }

    // TODO: Create a method for deleting a task
}
