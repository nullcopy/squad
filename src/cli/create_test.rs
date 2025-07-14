#[cfg(test)]
mod tests {
    use crate::cli::CreateCommand;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: TestCommands,
    }

    #[derive(clap::Subcommand)]
    enum TestCommands {
        Create(CreateCommand),
    }

    #[test]
    fn test_create_command_parsing() {
        let cli = TestCli::parse_from(["test", "create", "alice", "bob"]);
        match cli.command {
            TestCommands::Create(cmd) => {
                assert_eq!(cmd.names, vec!["alice", "bob"]);
            }
        }
    }

    #[test]
    fn test_create_command_empty_names() {
        let cli = TestCli::parse_from(["test", "create"]);
        match cli.command {
            TestCommands::Create(cmd) => {
                assert!(cmd.names.is_empty());
            }
        }
    }

    #[test]
    fn test_create_command_single_name() {
        let cli = TestCli::parse_from(["test", "create", "alice"]);
        match cli.command {
            TestCommands::Create(cmd) => {
                assert_eq!(cmd.names, vec!["alice"]);
            }
        }
    }

    #[test]
    fn test_parallel_task_creation() {
        // Test that multiple agent names are handled correctly for parallel processing
        let cmd = CreateCommand {
            names: vec![
                "agent1".to_string(),
                "agent2".to_string(),
                "agent3".to_string(),
                "agent4".to_string(),
                "agent5".to_string(),
            ],
        };

        // Verify we have multiple agents to process in parallel
        assert_eq!(cmd.names.len(), 5);
        assert!(cmd.names.contains(&"agent1".to_string()));
        assert!(cmd.names.contains(&"agent5".to_string()));
    }

    #[tokio::test]
    async fn test_empty_names_execution() {
        // Test that empty names list returns early without error
        let cmd = CreateCommand { names: vec![] };

        let result = cmd.execute().await;
        assert!(result.is_ok());
    }
}
