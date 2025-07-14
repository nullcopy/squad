use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::parser::parse_container_output;
use super::{ContainerRuntime, RuntimeType};

const AGENT_IMAGE: &str = "ubuntu:latest"; // Placeholder image for now
const AGENT_LABEL: &str = "squad.agent=true";
pub(crate) const MAX_AGENT_NAME_LENGTH: usize = 63; // Docker/Podman container name limit

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub created: DateTime<Local>,
}

#[derive(Debug, Clone)]
pub struct AgentDetails {
    pub id: String,
    pub name: String,
    pub status: String,
    pub created: String,
    pub ports: Option<String>,
}

impl Agent {
    pub async fn create(runtime: &ContainerRuntime, name: &str) -> Result<Self> {
        // Validate name
        if name.is_empty() {
            return Err(anyhow!("Agent name cannot be empty"));
        }

        if name.len() > MAX_AGENT_NAME_LENGTH {
            return Err(anyhow!(
                "Agent name cannot exceed {} characters",
                MAX_AGENT_NAME_LENGTH
            ));
        }

        if !name.chars().next().unwrap().is_alphabetic() {
            return Err(anyhow!("Agent name must start with a letter"));
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(anyhow!(
                "Agent name can only contain alphanumeric characters, hyphens, and underscores"
            ));
        }

        // Check if agent with this name already exists
        if Self::find_by_name(runtime, name).await?.is_some() {
            return Err(anyhow!("Agent '{}' already exists", name));
        }

        // Create the container with security constraints
        let output = runtime
            .run_command(&[
                "run",
                "-d",
                "--name",
                name,
                "--memory",
                "512m", // Memory limit
                "--cpus",
                "0.5", // CPU limit
                "--security-opt",
                "no-new-privileges", // Security hardening
                "--cap-drop",
                "ALL", // Drop all capabilities
                "--label",
                AGENT_LABEL,
                "--label",
                &format!("squad.agent.name={name}"),
                AGENT_IMAGE,
                "sleep",
                "infinity", // Keep container running
            ])
            .await?;

        let id = String::from_utf8_lossy(&output.stdout).trim().to_string();

        Ok(Self {
            id,
            name: name.to_string(),
            created: Local::now(),
        })
    }

    pub async fn list(runtime: &ContainerRuntime) -> Result<Vec<Self>> {
        let output = runtime
            .run_command_unchecked(&[
                "ps",
                "--filter",
                &format!("label={AGENT_LABEL}"),
                "--format",
                "{{.ID}}|{{.Names}}|{{.CreatedAt}}",
            ])
            .await?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let agents = parse_container_output(&stdout, |parts| {
            if parts.len() >= 3 {
                Some(Agent {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    created: Local::now(), // Simplified for now
                })
            } else {
                None
            }
        });

        Ok(agents)
    }

    pub async fn list_with_details(runtime: &ContainerRuntime) -> Result<Vec<AgentDetails>> {
        let format = match runtime.runtime_type {
            RuntimeType::Docker => "table {{.ID}}|{{.Names}}|{{.Status}}|{{.CreatedAt}}|{{.Ports}}",
            RuntimeType::Podman => "{{.ID}}|{{.Names}}|{{.Status}}|{{.Created}}|{{.Ports}}",
        };

        let output = runtime
            .run_command_unchecked(&[
                "ps",
                "--filter",
                &format!("label={AGENT_LABEL}"),
                "--format",
                format,
            ])
            .await?;

        if !output.status.success() {
            return Ok(Vec::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let agents: Vec<AgentDetails> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    Some(AgentDetails {
                        id: parts[0].to_string(),
                        name: parts[1].to_string(),
                        status: parts[2].to_string(),
                        created: parts[3].to_string(),
                        ports: if parts.len() > 4 && !parts[4].is_empty() {
                            Some(parts[4].to_string())
                        } else {
                            None
                        },
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(agents)
    }

    pub async fn find_by_name(runtime: &ContainerRuntime, name: &str) -> Result<Option<Self>> {
        let agents = Self::list(runtime).await?;
        Ok(agents.into_iter().find(|agent| agent.name == name))
    }

    pub async fn remove(&self, runtime: &ContainerRuntime) -> Result<()> {
        runtime.run_command(&["rm", "-f", &self.id]).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_agent_name_validation() {
        let valid_names = vec!["alice", "bob-1", "test_agent", "Agent123"];
        let invalid_names = vec![
            "",
            "alice@bob",
            "test agent",
            "agent/test",
            "123start",
            "_underscore",
        ];

        for name in valid_names {
            assert!(!name.is_empty());
            assert!(name.len() <= MAX_AGENT_NAME_LENGTH);
            assert!(name.chars().next().unwrap().is_alphabetic());
            assert!(name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
        }

        for name in invalid_names {
            let is_invalid = name.is_empty()
                || name.len() > MAX_AGENT_NAME_LENGTH
                || !name
                    .chars()
                    .next()
                    .map(|c| c.is_alphabetic())
                    .unwrap_or(false)
                || !name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_');
            assert!(is_invalid, "Name '{name}' should be invalid");
        }
    }
}

#[cfg(test)]
#[path = "agent_tests.rs"]
mod agent_tests;
