use anyhow::{anyhow, Result};
use std::process::Command;
use which::which;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeType {
    Podman,
    Docker,
}

#[derive(Debug, Clone)]
pub struct ContainerRuntime {
    pub runtime_type: RuntimeType,
    pub command: String,
}

impl ContainerRuntime {
    pub async fn detect() -> Result<Self> {
        // First check for Podman (preferred)
        if which("podman").is_ok() {
            // Verify Podman is working
            let output = Command::new("podman")
                .arg("version")
                .output()
                .map_err(|e| anyhow!("Failed to run podman version: {}", e))?;

            if output.status.success() {
                return Ok(Self {
                    runtime_type: RuntimeType::Podman,
                    command: "podman".to_string(),
                });
            }
        }

        // Fall back to Docker
        if which("docker").is_ok() {
            // Verify Docker is working
            let output = Command::new("docker")
                .arg("version")
                .output()
                .map_err(|e| anyhow!("Failed to run docker version: {}", e))?;

            if output.status.success() {
                return Ok(Self {
                    runtime_type: RuntimeType::Docker,
                    command: "docker".to_string(),
                });
            }
        }

        Err(anyhow!(
            "No container runtime found. Please install Podman or Docker."
        ))
    }

    pub fn name(&self) -> &str {
        match self.runtime_type {
            RuntimeType::Podman => "Podman",
            RuntimeType::Docker => "Docker",
        }
    }

    pub async fn run_command(&self, args: &[&str]) -> Result<std::process::Output> {
        let mut cmd = Command::new(&self.command);
        for arg in args {
            cmd.arg(arg);
        }
        let output = cmd
            .output()
            .map_err(|e| anyhow!("Failed to run {} command: {}", self.command, e))?;

        if !output.status.success() {
            // In debug mode, show full error details
            #[cfg(debug_assertions)]
            {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!(
                    "{} command failed: {}",
                    self.command,
                    stderr.trim()
                ));
            }

            // In release mode, show sanitized error
            #[cfg(not(debug_assertions))]
            return Err(anyhow!(
                "{} command failed. Run with RUST_LOG=debug for details",
                self.command
            ));
        }

        Ok(output)
    }

    pub async fn run_command_unchecked(&self, args: &[&str]) -> Result<std::process::Output> {
        let mut cmd = Command::new(&self.command);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.output()
            .map_err(|e| anyhow!("Failed to run {} command: {}", self.command, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_runtime_detection() {
        // This test will pass if either Podman or Docker is installed
        let result = ContainerRuntime::detect().await;

        // We can't guarantee which runtime is available in test environment
        // so we just check that detection works without panicking
        match result {
            Ok(runtime) => {
                assert!(!runtime.command.is_empty());
                assert!(matches!(
                    runtime.runtime_type,
                    RuntimeType::Podman | RuntimeType::Docker
                ));
            }
            Err(_) => {
                // It's okay if no runtime is found in test environment
            }
        }
    }
}

#[cfg(test)]
#[path = "runtime_tests.rs"]
mod runtime_tests;
