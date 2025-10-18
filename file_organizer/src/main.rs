mod cli;
mod organizer;
mod sorter;
mod utils;

use crate::cli::CliArgs;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    args.exec()?;
    Ok(())
}
