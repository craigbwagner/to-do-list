use crate::task::Task;

use color_eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Error, Write};

pub fn store_task(task: Task) -> Result<(), Error> {
    // serialize to JSON string
    let task_json = serde_json::to_string(&task)?;
    let mut file = File::create("tasks.json")?;

    file.write_all(task_json.as_bytes())?;

    Ok(())
}

pub fn read_tasks() -> HashMap<u32, Task> {
    let mut tasks_map = HashMap::new();
    if let Ok(lines) = read_lines() {
        for line in lines {
            let task: Task =
                serde_json::from_str(&line.unwrap()[..]).expect("Failed to parse json.");
            let task_id = task.id;

            tasks_map.insert(task_id, task);
        }
    }
    tasks_map
}

fn read_lines() -> std::result::Result<io::Lines<io::BufReader<File>>, Error> {
    let file = File::open("./tasks.json").expect("Failed to open file.");
    Ok(io::BufReader::new(file).lines())
}
