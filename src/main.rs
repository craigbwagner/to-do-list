mod storage;
mod task;

use crate::task::Task;

fn main() {
    let existing_tasks = storage::read_tasks();

    let name = "Dishes".to_string();
    let description = "General Description".to_string();
    let sample_task = Task::new(name, description);

    for task in existing_tasks {
        dbg!(task);
    }
}
