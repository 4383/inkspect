use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The config file to use
    #[arg(long)]
    pub config: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Show secrets in verbose logs
    #[arg(long)]
    pub show_secrets: bool,

    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    /// Optimize a prompt
    Optimize {
        /// The prompt to optimize
        #[arg(short, long)]
        input: Option<String>,

        /// The editor to use for input
        #[arg(short, long)]
        editor: Option<String>,

        /// The provider to use
        #[arg(short, long)]
        provider: Option<String>,

        /// The style to use
        #[arg(short, long)]
        style: Option<String>,

        /// The prompt to use
        #[arg(long)]
        prompt: Option<String>,

        /// The output file
        #[arg(short, long)]
        output: Option<String>,

        /// Disable the system prompt
        #[arg(long)]
        no_system_prompt: bool,
    },
    /// List available models from a provider
    ListModels {
        /// The provider to list models from
        #[arg(short, long)]
        provider: Option<String>,
    },
    /// List available prompts
    ListPrompts,
    /// Run the initial setup to create a configuration file
    Setup {
        /// The config file to create
        #[arg(long)]
        config: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_config() {
        let args = vec!["inkspect", "--config", "my_config.toml", "optimize"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.config, Some("my_config.toml".to_string()));
    }

    #[test]
    fn test_cli_parsing_input() {
        let args = vec!["inkspect", "optimize", "--input", "test prompt"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Optimize { input, .. } => {
                assert_eq!(input, Some("test prompt".to_string()));
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_cli_parsing_editor() {
        let args = vec!["inkspect", "optimize", "--editor", "vim"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Optimize { editor, .. } => {
                assert_eq!(editor, Some("vim".to_string()));
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_cli_parsing_provider() {
        let args = vec!["inkspect", "optimize", "--provider", "gemini"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Optimize { provider, .. } => {
                assert_eq!(provider, Some("gemini".to_string()));
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_cli_parsing_style() {
        let args = vec!["inkspect", "optimize", "--style", "refine"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Optimize { style, .. } => {
                assert_eq!(style, Some("refine".to_string()));
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_cli_parsing_output() {
        let args = vec!["inkspect", "optimize", "--output", "output.txt"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Commands::Optimize { output, .. } => {
                assert_eq!(output, Some("output.txt".to_string()));
            }
            _ => panic!("Expected Optimize command"),
        }
    }

    #[test]
    fn test_cli_parsing_optimize_command() {
        let args = vec!["inkspect", "optimize"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Commands::Optimize { .. }));
    }

    #[test]
    fn test_cli_parsing_list_prompts_command() {
        let args = vec!["inkspect", "list-prompts"];
        let cli = Cli::parse_from(args);
        assert!(matches!(cli.command, Commands::ListPrompts));
    }
}
