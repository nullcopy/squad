use anyhow::Result;
use clap::Args;
use colored::*;
use futures::future::join_all;

use super::utils::{print_error, print_success, print_warning};
use crate::container::{Agent, ContainerRuntime};

#[derive(Args)]
pub struct CreateCommand {
    #[arg(help = "Names of the agents to create")]
    pub(crate) names: Vec<String>,
}

impl CreateCommand {
    pub async fn execute(self) -> Result<()> {
        if self.names.is_empty() {
            print_warning("No agent names provided. Use: squad create <name1> [name2] ...");
            return Ok(());
        }

        let runtime = ContainerRuntime::detect().await?;
        println!("Using container runtime: {}", runtime.name().green());

        // Create all agents in parallel
        let creation_tasks = self.names.iter().map(|name| {
            let runtime = runtime.clone();
            let name = name.clone();
            tokio::spawn(async move {
                let result = Agent::create(&runtime, &name).await;
                (name, result)
            })
        });

        let results = join_all(creation_tasks).await;

        // Process results and display output
        for task_result in results {
            match task_result {
                Ok((_name, Ok(agent))) => {
                    print_success(
                        "Created agent",
                        &agent.name,
                        Some(&format!("ID: {}", agent.id)),
                    );
                }
                Ok((name, Err(e))) => {
                    print_error("create agent", &name, &e.to_string());
                }
                Err(join_error) => {
                    eprintln!("Task execution error: {join_error}");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "create_test.rs"]
mod create_test;
