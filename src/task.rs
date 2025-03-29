use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub enum Status {
    Incomplete,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    id: u32,
    name: String,
    description: String,
    status: Status,
}

static TASK_ID_COUNTER: OnceLock<AtomicU32> = OnceLock::new();

impl Task {
    pub fn new(name: String, description: String) -> Self {
        // initialize the counter if it hasn't already been initialized
        let counter = TASK_ID_COUNTER.get_or_init(|| AtomicU32::new(1));

        // generate a unique id by incrementing the counter
        let id = counter.fetch_add(1, Ordering::SeqCst);

        print!("Id is: {}", id);

        Self {
            id,
            name,
            description,
            status: Status::Incomplete,
        }
    }

    pub fn complete_task(&mut self) {
        self.status = Status::Completed;

        println!("Task '{}' marked as completed.", self.name);
    }

    // TODO: Create a method for deleting a task
}
