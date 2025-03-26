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

impl Task {
    pub fn create_task(name: String, description: String) -> Self {
        Self {
            id: 1,
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
