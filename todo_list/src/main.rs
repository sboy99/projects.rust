mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::CliArgs;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    args.exec()?;
    Ok(())
}
