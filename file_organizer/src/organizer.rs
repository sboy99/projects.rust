use crate::cli::SortMode;
use anyhow::Result;
use chrono::{DateTime, Local};
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn organize_files(path: &str, mode: SortMode, dry_run: bool) -> Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let src_path = entry.path();
            let dest_dir = match mode {
                SortMode::Extension => get_extension_dir(src_path),
                SortMode::Date => get_date_dir(src_path)?,
            };
            if dry_run {
                println!("Would move {:?} -> {:?}", src_path, dest_dir);
            } else {
                fs::create_dir_all(&dest_dir)?;
                let src_file_name = src_path.file_name().unwrap();
                fs::rename(src_path, dest_dir.join(src_file_name))?
            }
        }
    }
    Ok(())
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
