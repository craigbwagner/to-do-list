use crate::task::Task;
use serde_json::Result;

fn create_json(task: Task) -> Result<()> {
    let task_json = serde_json::to_string(&task);

    Ok(())
}
