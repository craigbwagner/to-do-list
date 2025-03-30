use crate::task::Task;
use serde_json::{Error, Result};

pub fn create_json(task: Task) -> Result<()> {
    // serialize to JSON string
    let task_json = serde_json::to_string(&task)?;

    // ? should I do a match pattern here on task_json?
    store_task(Ok(task_json));

    Ok(())
}

fn store_task(task: Result<String>) -> () {
    match task {
        Ok(task_string) => {}
        Err(e) => {}
    }
}
