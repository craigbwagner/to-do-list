#[derive(Debug)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Task {
    pub fn create_task(&self, name: String, description: String) -> () {}
}
