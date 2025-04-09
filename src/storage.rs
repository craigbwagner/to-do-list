use crate::task::Task;

use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind, Write};

pub fn store_task(task: Task) -> Result<(), Error> {
    // serialize to JSON string
    let task_json =
        serde_json::to_string(&task).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
    let mut file = File::create("tasks.json")?;

    file.write_all(task_json.as_bytes())?;

    Ok(())
}

pub fn read_tasks() -> Vec<Task> {
    let mut existing_tasks = Vec::new();
    if let Ok(lines) = read_lines() {
        for line in lines {
            let task = serde_json::from_str(&line.unwrap()[..]).expect("Failed to parse json.");

            existing_tasks.push(task);
        }
    }
    existing_tasks
}

fn read_lines() -> std::result::Result<io::Lines<io::BufReader<File>>, Error> {
    let file = File::open("./tasks.json").expect("Failed to open file.");
    Ok(io::BufReader::new(file).lines())
}
