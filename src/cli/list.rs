use anyhow::Result;
use clap::Args;
use colored::*;

use crate::container::{Agent, ContainerRuntime};

#[derive(Args)]
pub struct ListCommand {}

impl ListCommand {
    pub async fn execute(self) -> Result<()> {
        let runtime = ContainerRuntime::detect().await?;
        let agents = Agent::list(&runtime).await?;

        if agents.is_empty() {
            println!("{}", "No agents found.".yellow());
            return Ok(());
        }

        println!("{}", "Active Agents:".bold());
        for agent in agents {
            println!("  {} {}", "•".green(), agent.name.cyan());
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "list_test.rs"]
mod list_test;
