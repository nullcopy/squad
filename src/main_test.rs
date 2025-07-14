#[cfg(test)]
mod tests {
    use crate::{Cli, Commands};
    use clap::Parser;

    #[test]
    fn test_cli_parsing_create_command() {
        let args = vec!["squad", "create", "alice", "bob"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Create(cmd) => {
                assert_eq!(cmd.names, vec!["alice", "bob"]);
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parsing_create_command_single() {
        let args = vec!["squad", "create", "alice"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Create(cmd) => {
                assert_eq!(cmd.names, vec!["alice"]);
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parsing_create_command_no_args() {
        let args = vec!["squad", "create"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Create(cmd) => {
                assert!(cmd.names.is_empty());
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parsing_list_command() {
        let args = vec!["squad", "list"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::List(_) => {}
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_parsing_rm_command() {
        let args = vec!["squad", "rm", "alice"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Rm(cmd) => {
                assert_eq!(cmd.names, vec!["alice"]);
                assert!(!cmd.all);
            }
            _ => panic!("Expected Rm command"),
        }
    }

    #[test]
    fn test_cli_parsing_rm_command_all() {
        let args = vec!["squad", "rm", "--all"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Rm(cmd) => {
                assert!(cmd.names.is_empty());
                assert!(cmd.all);
            }
            _ => panic!("Expected Rm command"),
        }
    }

    #[test]
    fn test_cli_parsing_rm_command_all_short() {
        let args = vec!["squad", "rm", "-a"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Rm(cmd) => {
                assert!(cmd.names.is_empty());
                assert!(cmd.all);
            }
            _ => panic!("Expected Rm command"),
        }
    }

    #[test]
    fn test_cli_parsing_ps_command() {
        let args = vec!["squad", "ps"];
        let cli = Cli::try_parse_from(args).unwrap();

        match cli.command {
            Commands::Ps(_) => {}
            _ => panic!("Expected Ps command"),
        }
    }

    #[test]
    fn test_cli_parsing_help() {
        let args = vec!["squad", "--help"];
        let result = Cli::try_parse_from(args);

        // Help should cause an error (but it's expected)
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_version() {
        let args = vec!["squad", "--version"];
        let result = Cli::try_parse_from(args);

        // Version should cause an error (but it's expected)
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_invalid_command() {
        let args = vec!["squad", "invalid"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parsing_no_args() {
        let args = vec!["squad"];
        let result = Cli::try_parse_from(args);

        // Should fail because no subcommand provided
        assert!(result.is_err());
    }

    #[test]
    fn test_commands_enum_coverage() {
        // Test that all Commands variants are covered
        let create_args = vec!["squad", "create"];
        let list_args = vec!["squad", "list"];
        let rm_args = vec!["squad", "rm"];
        let ps_args = vec!["squad", "ps"];

        assert!(Cli::try_parse_from(create_args).is_ok());
        assert!(Cli::try_parse_from(list_args).is_ok());
        assert!(Cli::try_parse_from(rm_args).is_ok());
        assert!(Cli::try_parse_from(ps_args).is_ok());
    }
}
