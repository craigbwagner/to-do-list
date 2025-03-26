mod task;

use crate::task::{Status, Task};

fn main() {
    let mut sample_task = Task {
        id: 1,
        name: "Do dishes".to_string(),
        description: "Obvious description".to_string(),
        status: Status::NotStarted,
    };

    print!("{:#?}", sample_task)
}
