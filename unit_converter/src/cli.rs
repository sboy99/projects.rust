use crate::converters;
use crate::interactive;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "unit_converter", about = "Convert units between different systems", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Temperature {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    Length {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    Weight {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        value: f64,
    },
    Interactive,
}

impl CliArgs {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Command::Temperature { from, to, value } => {
                let result = converters::convert_temperature(from, to, value)?;
                println!("{} {} = {} {}", value, from, result, to);
            }
            Command::Length { from, to, value } => {
                let result = converters::convert_length(from, to, value)?;
                println!("{} {} = {} {}", value, from, result, to);
            }
            Command::Weight { from, to, value } => {
                let result = converters::convert_weight(from, to, value)?;
                println!("{} {} = {} {}", value, from, result, to);
            }
            Command::Interactive => {
                interactive::run()?;
            }
        }
        Ok(())
    }
}
