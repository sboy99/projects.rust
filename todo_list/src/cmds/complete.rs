use crate::storage;
use anyhow::Result;

pub fn exec(id: &usize) -> Result<()> {
    let mut is_found: bool = false;
    let mut tasks = storage::list_tasks()?;
    for task in &mut tasks {
        if task.id == *id as u32 {
            if !task.completed {
                task.complete();
                is_found = true;
            }
            println!("Task: '{}' is completed", task.title);
            break;
        }
    }
    if !is_found {
        return Err(anyhow::anyhow!("Task with ID {} not found", id));
    }
    storage::save_tasks(&tasks)?;
    Ok(())
}
