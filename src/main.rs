mod storage;
mod task;

use crate::task::Task;

fn main() {
    let name = "Dishes".to_string();
    let description = "General Description".to_string();
    let mut sample_task = Task::new(name, description);
    Task::complete(&mut sample_task);

    println!("{:#?}", sample_task);
}
