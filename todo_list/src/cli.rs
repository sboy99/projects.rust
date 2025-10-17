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
            Commands::Add { title } => crate::cmds::add::exec(title),
            Commands::List => crate::cmds::list::exec(),
            Commands::Remove { id } => crate::cmds::remove::exec(id),
            Commands::Complete { id } => crate::cmds::complete::exec(id),
        }
    }
}
