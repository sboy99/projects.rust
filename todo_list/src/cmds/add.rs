use crate::models::Task;
use crate::storage;
use anyhow::Result;

pub fn exec(title: &String) -> Result<()> {
    let id = storage::generate_id()?;
    let new_task = Task::new(id, title.clone());
    let mut tasks = storage::list_tasks()?;
    tasks.push(new_task.clone());
    storage::save_tasks(&tasks)?;
    println!("Task '{}' added successfully!", new_task.title);
    Ok(())
}
