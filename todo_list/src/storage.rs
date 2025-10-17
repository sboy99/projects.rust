use crate::models::Task;
use anyhow::Result;
use std::fs;

const FILE_PATH: &str = "data/tasks.json";
const ID_COUNTER_PATH: &str = "data/id_counter.txt";

pub fn list_tasks() -> Result<Vec<Task>> {
    if let Ok(content) = fs::read_to_string(FILE_PATH) {
        let tasks: Vec<Task> = serde_json::from_str(&content)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    fs::create_dir_all("data")?;
    let content = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE_PATH, content)?;
    Ok(())
}

pub fn generate_id() -> Result<u32> {
    let mut counter = 0;
    if let Ok(content) = fs::read_to_string(ID_COUNTER_PATH) {
        counter = content.parse()?;
    }
    counter += 1;
    fs::write(ID_COUNTER_PATH, counter.to_string())?;
    Ok(counter)
}
