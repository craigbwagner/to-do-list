#[derive(Debug)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    id: u32,
    name: String,
    description: String,
    completed: Status,
}

impl Task {
    fn new(&self, name: String, description: String) -> Self {
        Task {
            name,
            description,
            completed: Status::NotStarted,
        }
    }
}
