use crate::storage;
use anyhow::Result;

pub fn exec() -> Result<()> {
    let tasks = storage::list_tasks()?;
    for task in tasks {
        println!("Task ID: '{}', Title: {}", task.id, task.title);
    }
    Ok(())
}
