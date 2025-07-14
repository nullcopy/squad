use anyhow::Result;
use clap::Args;
use futures::future::join_all;

use super::utils::{print_error, print_success, print_warning};
use crate::container::{Agent, ContainerRuntime};

#[derive(Args)]
pub struct RmCommand {
    #[arg(help = "Names of the agents to remove")]
    pub(crate) names: Vec<String>,

    #[arg(short, long, help = "Remove all agents")]
    pub(crate) all: bool,
}

impl RmCommand {
    pub async fn execute(self) -> Result<()> {
        let runtime = ContainerRuntime::detect().await?;

        if self.all {
            let agents = Agent::list(&runtime).await?;
            if agents.is_empty() {
                print_warning("No agents to remove.");
                return Ok(());
            }

            println!("Removing all {} agents...", agents.len());

            // Remove all agents in parallel
            let removal_tasks = agents.into_iter().map(|agent| {
                let runtime = runtime.clone();
                tokio::spawn(async move {
                    let name = agent.name.clone();
                    let result = agent.remove(&runtime).await;
                    (name, result)
                })
            });

            let results = join_all(removal_tasks).await;

            // Process results and display output
            for task_result in results {
                match task_result {
                    Ok((name, Ok(_))) => {
                        print_success("Removed agent", &name, None);
                    }
                    Ok((name, Err(e))) => {
                        print_error("remove agent", &name, &e.to_string());
                    }
                    Err(join_error) => {
                        eprintln!("Task execution error: {join_error}");
                    }
                }
            }
        } else if self.names.is_empty() {
            print_warning(
                "No agent names provided. Use: squad rm <name1> [name2] ... or squad rm --all",
            );
        } else {
            // Remove specific agents in parallel
            let removal_tasks = self.names.iter().map(|name| {
                let runtime = runtime.clone();
                let name = name.clone();
                tokio::spawn(async move {
                    let result = match Agent::find_by_name(&runtime, &name).await {
                        Ok(Some(agent)) => agent.remove(&runtime).await.map(|_| ()),
                        Ok(None) => Err(anyhow::anyhow!("Agent '{}' not found", name)),
                        Err(e) => Err(anyhow::anyhow!("Error finding agent '{}': {}", name, e)),
                    };
                    (name, result)
                })
            });

            let results = join_all(removal_tasks).await;

            // Process results and display output
            for task_result in results {
                match task_result {
                    Ok((name, Ok(_))) => {
                        print_success("Removed agent", &name, None);
                    }
                    Ok((name, Err(e))) => {
                        print_error("remove agent", &name, &e.to_string());
                    }
                    Err(join_error) => {
                        eprintln!("Task execution error: {join_error}");
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "rm_test.rs"]
mod rm_test;
