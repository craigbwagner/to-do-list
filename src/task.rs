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
    status: Status,
}

impl Task {
    pub fn create_task(name: String, description: String) -> Self {
        Self {
            id: 1,
            name,
            description,
            status: Status::NotStarted,
        }
    }
}
