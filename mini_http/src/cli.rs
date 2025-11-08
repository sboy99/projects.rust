use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cmds::{delete, get, patch, post};

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get {
        url: String,
        #[arg(long = "headers")]
        headers: Option<String>,
    },
    Post {
        url: String,
        #[arg(long = "headers")]
        headers: Option<String>,
        #[arg(long = "body")]
        body: Option<String>,
    },
    Patch {
        url: String,
        #[arg(long = "headers")]
        headers: Option<String>,
        #[arg(long = "body")]
        body: Option<String>,
    },
    Delete {
        url: String,
        #[arg(long = "headers")]
        headers: Option<String>,
    },
}

impl CliArgs {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Command::Get { url, headers } => get(url.clone(), headers.clone()).await,
            Command::Post { url, headers, body } => {
                post(url.clone(), body.clone(), headers.clone()).await
            }
            Command::Patch { url, headers, body } => {
                patch(url.clone(), body.clone(), headers.clone()).await
            }
            Command::Delete { url, headers } => delete(url.clone(), headers.clone()).await,
        }
    }
}
