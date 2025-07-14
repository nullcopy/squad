use anyhow::Result;
use clap::Args;
use colored::*;
use tabled::{builder::Builder, settings::Style};

use crate::container::{Agent, ContainerRuntime};

#[derive(Args)]
pub struct PsCommand {}

impl PsCommand {
    pub async fn execute(self) -> Result<()> {
        let runtime = ContainerRuntime::detect().await?;
        let agents = Agent::list_with_details(&runtime).await?;

        if agents.is_empty() {
            println!("{}", "No agents found.".yellow());
            return Ok(());
        }

        let mut builder = Builder::default();
        builder.push_record(["NAME", "ID", "STATUS", "CREATED", "PORTS"]);

        for agent in agents {
            builder.push_record([
                &agent.name,
                &agent.id[..12], // Show first 12 chars of ID
                &agent.status,
                &agent.created,
                &agent.ports.unwrap_or_else(|| "-".to_string()),
            ]);
        }

        let mut table = builder.build();
        table.with(Style::modern());
        println!("{table}");

        Ok(())
    }
}

#[cfg(test)]
#[path = "ps_test.rs"]
mod ps_test;
