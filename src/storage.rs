use crate::task::Task;
use serde_json::Result;

use std::fs;

pub fn create_json(task: Task) -> Result<()> {
    // serialize to JSON string
    let task_json = serde_json::to_string(&task)?;

    store_task(task_json);

    Ok(())
}

pub fn store_task(task: String) -> () {
    fs::write("../tasks.txt", task).expect("Unable to write file");
}
