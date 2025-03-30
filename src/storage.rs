use crate::task::Task;
use serde_json::Result;

use std::fs::File;
use std::io::Write;

pub fn store_task(task: Task) -> Result<()> {
    // serialize to JSON string
    let task_json = serde_json::to_string(&task)?;
    let mut file = File::create("tasks.json").expect("Failed to create file");

    file.write(task_json.as_bytes())
        .expect("Unable to write file");

    Ok(())
}
