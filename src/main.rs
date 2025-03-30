mod storage;
mod task;

use storage::create_json;

use crate::task::Task;

fn main() {
    let name = "Dishes".to_string();
    let description = "General Description".to_string();
    let sample_task = Task::new(name, description);

    create_json(sample_task).expect("Failed to create task");
}
