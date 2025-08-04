use super::cli::{Cli, Commands};
use super::config::Config;
use super::llm::r#trait::LlmBackend;
use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub async fn run(cli: Cli, config: Config, llm_backend: Box<dyn LlmBackend>) -> Result<()> {
    match cli.command {
        Commands::Optimize {
            input,
            editor,
            style,
            prompt: dynamic_prompt,
            output,
            no_system_prompt,
            ..
        } => {
            let style_text = if let Some(p) = dynamic_prompt {
                p
            } else {
                let style_key = style.as_deref().unwrap_or(&config.llm.default_prompt);
                let prompt_style = config
                    .prompts
                    .iter()
                    .find(|p| p.name == style_key)
                    .ok_or_else(|| {
                        anyhow::anyhow!("Prompt style '{}' not found in configuration.", style_key)
                    })?;
                prompt_style.prompt.clone()
            };

            let prompt = if let Some(input) = input {
                input
            } else {
                let temp_file = tempfile::NamedTempFile::new()?;
                let editor_cmd = editor
                    .or_else(|| std::env::var("EDITOR").ok())
                    .unwrap_or_else(|| "vim".to_string());
                super::editor::open_editor(temp_file.path(), &editor_cmd)?;
                super::editor::read_editor_input(temp_file.path())?
            };

            if prompt.trim().is_empty() {
                eprintln!("Input is empty. Exiting.");
                return Ok(());
            }

            let full_prompt = if !no_system_prompt && config.llm.system_prompt.is_some() {
                format!(
                    "{}

{}

{}",
                    config.llm.system_prompt.as_ref().unwrap(),
                    style_text,
                    prompt
                )
            } else {
                format!(
                    "{}

{}",
                    style_text, prompt
                )
            };

            log::debug!(
                "Using full prompt:
---
{}
---",
                full_prompt
            );

            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner:.rgb(181,126,220)} {msg:.rgb(181,126,220)}")?,
            );
            spinner.set_message("Optimizing prompt, please wait...");
            spinner.enable_steady_tick(Duration::from_millis(100));

            let response = llm_backend.request(&full_prompt).await;

            spinner.finish_and_clear();

            let response = response?;

            let lines: Vec<&str> = response.lines().collect();
            let first_line = lines.get(0).unwrap_or(&"");

            let prefixes_to_remove = [
                "Of course.",
                "Certainly.",
                "Here is a refined and comprehensive explanation",
                "Here's a refined and comprehensive explanation",
                "Here is a refined version",
                "Here's a refined version",
            ];

            let mut output_response = response.clone();
            if prefixes_to_remove.iter().any(|p| first_line.contains(p)) {
                output_response = lines
                    .iter()
                    .skip(1)
                    .map(|s| *s)
                    .collect::<Vec<&str>>()
                    .join("\n");
            }

            if let Some(output_path_str) = output {
                let output_path = std::path::Path::new(&output_path_str);
                let absolute_path = if output_path.is_absolute() {
                    output_path.to_path_buf()
                } else {
                    std::env::current_dir()?.join(output_path)
                };
                log::debug!("Saving output to: {}", absolute_path.display());
                std::fs::write(&absolute_path, output_response)?;
                log::debug!("Successfully wrote to {}", absolute_path.display());
            } else {
                println!("{}", output_response);
            }
        }
        Commands::ListModels { .. } => {
            let models = llm_backend.list_models().await?;
            for model in models {
                println!("{}", model);
            }
        }
        Commands::ListPrompts => {
            println!("{}", style("Available Prompts").bold().underlined());

            let mut prompts = config.prompts.clone();
            prompts.sort_by_key(|p| p.name.clone());

            for prompt in prompts {
                println!("\n{}", style(prompt.name).bold().cyan());
                if let Some(description) = prompt.description {
                    println!("  {}", description);
                }
            }
        }
        Commands::Setup { .. } => unreachable!(),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Cli;
    use crate::config::Config;
    use clap::Parser;

    struct MockLlmBackend;

    #[async_trait::async_trait]
    impl LlmBackend for MockLlmBackend {
        async fn request(&self, _full_prompt: &str) -> Result<String> {
            Ok("Mocked response".to_string())
        }

        async fn list_models(&self) -> Result<Vec<String>> {
            Ok(vec!["model1".to_string(), "model2".to_string()])
        }
    }

    #[tokio::test]
    async fn test_run_optimize_with_input() {
        let cli = Cli::parse_from(vec!["inkspect", "optimize", "--input", "test prompt"]);
        let config = Config::default();
        let llm_backend = Box::new(MockLlmBackend);
        let result = run(cli, config, llm_backend).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_list_models() {
        let cli = Cli::parse_from(vec!["inkspect", "list-models"]);
        let config = Config::default();
        let llm_backend = Box::new(MockLlmBackend);
        let result = run(cli, config, llm_backend).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_list_prompts() {
        let cli = Cli::parse_from(vec!["inkspect", "list-prompts"]);
        let config = Config::default();
        let llm_backend = Box::new(MockLlmBackend);
        let result = run(cli, config, llm_backend).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_optimize_with_empty_input() {
        let cli = Cli::parse_from(vec!["inkspect", "optimize", "--input", ""]);
        let config = Config::default();
        let llm_backend = Box::new(MockLlmBackend);
        let result = run(cli, config, llm_backend).await;
        assert!(result.is_ok());
    }
}
