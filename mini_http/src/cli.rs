use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cmds::get;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get {
        url: String,
        #[arg(long)]
        headers: Option<String>,
    },
    Post {
        url: String,
        #[arg(long)]
        headers: Option<String>,
        #[arg(long)]
        body: Option<String>,
    },
    Patch {
        url: String,
        #[arg(long)]
        headers: Option<String>,
        #[arg(long)]
        body: Option<String>,
    },
    Delete {
        url: String,
        #[arg(long)]
        headers: Option<String>,
    },
}

impl CliArgs {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Command::Get { url, headers } => get(url.clone(), headers.clone()).await,
            Command::Post { url, headers, body } => todo!(),
            Command::Patch { url, headers, body } => todo!(),
            Command::Delete { url, headers } => todo!(),
        }
    }
}
