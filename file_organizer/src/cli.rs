use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "file-organizer", about = "Sorts files by extension or date", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, default_value = ".")]
    pub path: String,

    #[arg(short, long, value_enum, default_value = "extension")]
    pub by: SortMode,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum SortMode {
    #[value(name = "extension")]
    Extension,
    #[value(name = "date")]
    Date,
}

impl CliArgs {
    pub fn exec(&self) -> Result<()> {
        Ok(())
    }
}
