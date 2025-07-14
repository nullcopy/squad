#[cfg(test)]
mod tests {
    use crate::cli::PsCommand;
    use clap::Parser;

    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: TestCommands,
    }

    #[derive(clap::Subcommand)]
    enum TestCommands {
        Ps(PsCommand),
    }

    #[test]
    fn test_ps_command_parsing() {
        let cli = TestCli::parse_from(["test", "ps"]);
        match cli.command {
            TestCommands::Ps(_) => {
                // Ps command has no arguments currently
            }
        }
    }
}
