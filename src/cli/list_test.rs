#[cfg(test)]
mod tests {
    use crate::cli::ListCommand;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: TestCommands,
    }

    #[derive(clap::Subcommand)]
    enum TestCommands {
        List(ListCommand),
    }

    #[test]
    fn test_list_command_parsing() {
        let cli = TestCli::parse_from(["test", "list"]);
        match cli.command {
            TestCommands::List(_) => {
                // List command has no arguments
            }
        }
    }
}
