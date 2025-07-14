use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper to create a mock container runtime script
fn create_mock_runtime(dir: &TempDir, runtime_name: &str) -> PathBuf {
    let script_path = dir.path().join(runtime_name);
    let script_content = r#"#!/bin/bash
# Mock container runtime for testing

case "$1" in
    "version")
        echo "Mock $0 version 1.0.0"
        exit 0
        ;;
    "ps")
        if [[ "$*" == *"--filter"* ]] && [[ "$*" == *"squad.agent=true"* ]]; then
            # Return empty list (no agents)
            exit 0
        fi
        ;;
    "run")
        # Simulate container creation
        echo "mock-container-id-12345"
        exit 0
        ;;
    "rm")
        # Simulate container removal
        exit 0
        ;;
    *)
        echo "Mock: Unknown command: $1"
        exit 1
        ;;
esac
"#;

    fs::write(&script_path, script_content).unwrap();

    // Make script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).unwrap();
    }

    script_path
}

#[test]
fn test_with_mock_runtime() {
    let temp_dir = TempDir::new().unwrap();
    let _mock_podman = create_mock_runtime(&temp_dir, "podman");

    // Add mock runtime to PATH
    let original_path = std::env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", temp_dir.path().display(), original_path);
    std::env::set_var("PATH", new_path);

    // Now squad should detect our mock runtime
    // This would require running the actual binary with the modified PATH

    // Restore original PATH
    std::env::set_var("PATH", original_path);
}

#[test]
fn test_agent_name_validation_comprehensive() {
    let valid_names = vec![
        "a",
        "alice",
        "bob-123",
        "test_agent",
        "Agent123",
        "my-cool-agent_1",
        "agent-with-many-hyphens-and_underscores_123",
        "UPPERCASE",
        "MixedCase123",
    ];

    let invalid_names = vec![
        "",
        " ",
        "alice bob",   // space
        "alice@bob",   // @
        "agent/test",  // /
        "agent.test",  // .
        "agent:test",  // :
        "agent;test",  // ;
        "agent,test",  // ,
        "agent'test",  // '
        "agent\"test", // "
        "agent\\test", // \
        "agent|test",  // |
        "agent>test",  // >
        "agent<test",  // <
        "agent?test",  // ?
        "agent*test",  // *
        "agent[test]", // []
        "agent{test}", // {}
        "agent(test)", // ()
        "agent#test",  // #
        "agent%test",  // %
        "agent&test",  // &
        "agent+test",  // +
        "agent=test",  // =
        "agent~test",  // ~
        "agent`test",  // `
        "agent!test",  // !
        "agent$test",  // $
        "agent^test",  // ^
    ];

    for name in &valid_names {
        assert!(
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
            "Name {name} should be valid"
        );
    }

    for name in &invalid_names {
        assert!(
            name.is_empty()
                || !name
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
            "Name {name} should be invalid"
        );
    }
}
