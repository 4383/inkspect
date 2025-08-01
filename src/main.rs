mod cli;
mod config;
mod core;
mod editor;
mod llm;
mod setup;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use llm::claude::ClaudeBackend;
use llm::gemini::GeminiBackend;
use llm::r#trait::LlmBackend;
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Setup = cli.command {
        return setup::setup();
    }

    env_logger::Builder::new()
        .filter_level(if cli.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();

    let (config, config_path): (Config, String) = if let Some(config_path) = &cli.config {
        (confy::load_path(config_path)?, config_path.clone())
    } else {
        let path = confy::get_configuration_file_path("inkspect", None)?;
        (
            confy::load("inkspect", None)?,
            path.to_string_lossy().into_owned(),
        )
    };

    if cli.verbose {
        log::debug!("Loaded config from: {}", config_path);
        let config_to_log = if cli.show_secrets {
            config.clone()
        } else {
            config.sanitized()
        };
        log::debug!("Loaded config: {:?}", config_to_log);
    }

    let llm_backend: Box<dyn LlmBackend> = match &cli.command {
        Commands::Optimize { provider, .. } => {
            let provider = provider.as_deref().unwrap_or(&config.llm.provider);
            match provider {
                "gemini" => Box::new(GeminiBackend::new(
                    config.providers.gemini.api_key.clone(),
                    config.providers.gemini.model.clone(),
                )),
                "claude" => Box::new(ClaudeBackend::new(
                    config.providers.claude.api_key.clone(),
                    config.providers.claude.model.clone(),
                )),
                _ => return Err(anyhow::anyhow!("Unsupported provider")),
            }
        }
        Commands::ListModels { provider } => {
            let provider = provider.as_deref().unwrap_or(&config.llm.provider);
            match provider {
                "gemini" => Box::new(GeminiBackend::new(
                    config.providers.gemini.api_key.clone(),
                    config.providers.gemini.model.clone(),
                )),
                "claude" => Box::new(ClaudeBackend::new(
                    config.providers.claude.api_key.clone(),
                    config.providers.claude.model.clone(),
                )),
                _ => return Err(anyhow::anyhow!("Unsupported provider")),
            }
        }
        Commands::ListStyles => {
            // The ListStyles command does not require a provider.
            Box::new(llm::gemini::GeminiBackend::new(
                "".to_string(),
                "".to_string(),
            ))
        }
        Commands::Setup => unreachable!(), // This is handled above
    };

    core::run(cli, config, llm_backend).await
}
