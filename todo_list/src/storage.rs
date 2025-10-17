use crate::models::Task;
use anyhow::Result;
use std::fs;

const FILE_PATH: &str = "data/tasks.json";

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
