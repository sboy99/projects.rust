mod cli;
mod cmds;
mod http;
mod printer;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::CliArgs;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args = CliArgs::parse();
    cli_args.run().await?;
    Ok(())
}
