use anyhow::Result;
use chrono::{DateTime, Local};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::SortMode;

pub fn get_dest_dir(path: &Path, mode: SortMode) -> Result<PathBuf> {
    match mode {
        SortMode::Extension => Ok(get_extension_dir(path)),
        SortMode::Date => get_date_dir(path),
    }
}

fn get_extension_dir(file_path: &Path) -> PathBuf {
    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown");
    Path::new("sorted").join(ext)
}

fn get_date_dir(file_path: &Path) -> Result<PathBuf> {
    let metadata = fs::metadata(file_path)?;
    let modified_time = metadata.modified()?;
    let datetime: DateTime<Local> = modified_time.into();
    let formatted_date = datetime.format("%Y/%B").to_string();
    Ok(Path::new("sorted").join(formatted_date))
}
