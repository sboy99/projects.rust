use crate::storage;
use anyhow::Result;

pub fn exec(id: &usize) -> Result<()> {
    let mut tasks = storage::list_tasks()?;
    tasks.retain(|task| task.id != *id as u32); // like lambda function in python or filter function in javascript
    storage::save_tasks(&tasks)?;
    Ok(())
}
