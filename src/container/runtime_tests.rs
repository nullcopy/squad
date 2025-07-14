#[cfg(test)]
mod tests {
    use super::super::*;
    #[allow(unused_imports)]
    use std::process::{Command, Stdio};

    #[test]
    fn test_runtime_type_equality() {
        assert_eq!(RuntimeType::Podman, RuntimeType::Podman);
        assert_eq!(RuntimeType::Docker, RuntimeType::Docker);
        assert_ne!(RuntimeType::Podman, RuntimeType::Docker);
    }

    #[test]
    fn test_runtime_name() {
        let podman_runtime = ContainerRuntime {
            runtime_type: RuntimeType::Podman,
            command: "podman".to_string(),
        };
        assert_eq!(podman_runtime.name(), "Podman");

        let docker_runtime = ContainerRuntime {
            runtime_type: RuntimeType::Docker,
            command: "docker".to_string(),
        };
        assert_eq!(docker_runtime.name(), "Docker");
    }

    #[tokio::test]
    async fn test_run_command_with_mock() {
        // Create a mock runtime that uses echo instead of actual container runtime
        let mock_runtime = ContainerRuntime {
            runtime_type: RuntimeType::Docker,
            command: "echo".to_string(),
        };

        let result = mock_runtime.run_command(&["test", "output"]).await;
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "test output"
        );
    }

    #[tokio::test]
    async fn test_run_command_unchecked() {
        let mock_runtime = ContainerRuntime {
            runtime_type: RuntimeType::Docker,
            command: "false".to_string(), // Command that always fails
        };

        let result = mock_runtime.run_command_unchecked(&[]).await;
        assert!(result.is_ok()); // Should not return error even if command fails

        let output = result.unwrap();
        assert!(!output.status.success()); // But status should indicate failure
    }
}
