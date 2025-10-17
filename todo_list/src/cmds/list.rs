use crate::storage;
use anyhow::Result;

pub fn exec() -> Result<()> {
    let tasks = storage::list_tasks()?;
    for task in tasks {
        println!("{:#?}", task);
    }
    Ok(())
}
