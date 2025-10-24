mod cli;
mod converters;
mod interactive;

use crate::cli::CliArgs;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    args.execute()?;
    Ok(())
}
