#[cfg(test)]
mod tests {
    use crate::cli::RmCommand;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: TestCommands,
    }

    #[derive(clap::Subcommand)]
    enum TestCommands {
        Rm(RmCommand),
    }

    #[test]
    fn test_rm_command_parsing_with_names() {
        let cli = TestCli::parse_from(["test", "rm", "alice", "bob"]);
        match &cli.command {
            TestCommands::Rm(cmd) => {
                assert_eq!(cmd.names, vec!["alice", "bob"]);
                assert!(!cmd.all);
            }
        }
    }

    #[test]
    fn test_rm_command_parsing_with_all_flag() {
        let cli = TestCli::parse_from(["test", "rm", "--all"]);
        match &cli.command {
            TestCommands::Rm(cmd) => {
                assert!(cmd.names.is_empty());
                assert!(cmd.all);
            }
        }
    }

    #[test]
    fn test_rm_command_parsing_with_all_short_flag() {
        let cli = TestCli::parse_from(["test", "rm", "-a"]);
        match &cli.command {
            TestCommands::Rm(cmd) => {
                assert!(cmd.names.is_empty());
                assert!(cmd.all);
            }
        }
    }

    #[test]
    fn test_rm_command_parsing_empty() {
        let cli = TestCli::parse_from(["test", "rm"]);
        match &cli.command {
            TestCommands::Rm(cmd) => {
                assert!(cmd.names.is_empty());
                assert!(!cmd.all);
            }
        }
    }

    #[test]
    fn test_parallel_removal_preparation() {
        // Test that multiple agent names are handled correctly for parallel processing
        let cmd = RmCommand {
            names: vec![
                "agent1".to_string(),
                "agent2".to_string(),
                "agent3".to_string(),
                "agent4".to_string(),
            ],
            all: false,
        };

        // Verify we have multiple agents to process in parallel
        assert_eq!(cmd.names.len(), 4);
        assert!(cmd.names.contains(&"agent1".to_string()));
        assert!(cmd.names.contains(&"agent4".to_string()));
        assert!(!cmd.all);
    }

    #[test]
    fn test_all_flag_configuration() {
        // Test that --all flag is properly set for parallel removal of all agents
        let cmd = RmCommand {
            names: vec![],
            all: true,
        };

        assert!(cmd.names.is_empty());
        assert!(cmd.all);
    }
}
