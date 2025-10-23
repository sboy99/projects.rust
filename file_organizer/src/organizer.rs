use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

use crate::cli::SortMode;
use crate::sorter::get_dest_dir;

pub fn organize_files(path: &str, mode: SortMode, dry_run: bool) -> Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let src_path = entry.path();
            let dest_dir = get_dest_dir(src_path, mode)?;
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
