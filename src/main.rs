use anyhow::Result;
use clap::{Parser, Subcommand};

mod cli;
mod container;

#[cfg(test)]
mod main_test;

use cli::{CreateCommand, ListCommand, PsCommand, RmCommand};

#[derive(Parser)]
#[command(name = "squad")]
#[command(about = "Assemble your squad of AI agents")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create and start new agent containers")]
    Create(CreateCommand),
    #[command(about = "List all active agents")]
    List(ListCommand),
    #[command(about = "Stop and remove agent containers")]
    Rm(RmCommand),
    #[command(about = "Show detailed status information for all agents")]
    Ps(PsCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create(cmd) => cmd.execute().await,
        Commands::List(cmd) => cmd.execute().await,
        Commands::Rm(cmd) => cmd.execute().await,
        Commands::Ps(cmd) => cmd.execute().await,
    }
}
