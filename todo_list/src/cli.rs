use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo",about = "A simple CLI to manage your tasks", long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List,
    Remove { id: usize },
    Complete { id: usize },
}

impl CliArgs {
    pub fn exec(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Add { title } => {
                // Add logic here
                println!("Adding task: {}", title);
                Ok(())
            }
            Commands::List => {
                // Add logic here
                println!("Listing tasks");
                Ok(())
            }
            Commands::Remove { id } => {
                // Add logic here
                println!("Removing task with ID: {}", id);
                Ok(())
            }
            Commands::Complete { id } => {
                // Add logic here
                println!("Completing task with ID: {}", id);
                println!("Task completed");
                Ok(())
            }
        }
    }
}
