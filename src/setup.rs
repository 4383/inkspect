use crate::config::Config;
use anyhow::Result;
use console::style;
use std::io::{self, Write};

pub fn setup() -> Result<()> {
    println!(
        "{}",
        style("Inkspect Configuration Setup").bold().underlined()
    );

    let config_path = confy::get_configuration_file_path("inkspect", None)?;
    println!(
        "This will create a new configuration file at: {}",
        style(config_path.display()).cyan()
    );

    if config_path.exists() {
        print!("A configuration file already exists. Do you want to overwrite it? (y/N) ");
        io::stdout().flush()?;
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;
        if confirmation.trim().to_lowercase() != "y" {
            println!("Setup cancelled.");
            return Ok(());
        }
    }

    let mut config = Config::default();

    print!("Enter your Gemini API Key (leave blank to skip): ");
    io::stdout().flush()?;
    let mut gemini_key = String::new();
    io::stdin().read_line(&mut gemini_key)?;
    if !gemini_key.trim().is_empty() {
        config.providers.gemini.api_key = gemini_key.trim().to_string();
    }

    print!("Enter your Claude API Key (leave blank to skip): ");
    io::stdout().flush()?;
    let mut claude_key = String::new();
    io::stdin().read_line(&mut claude_key)?;
    if !claude_key.trim().is_empty() {
        config.providers.claude.api_key = claude_key.trim().to_string();
    }

    confy::store_path(&config_path, config)?;

    println!(
        "\n{}",
        style("Configuration file created successfully!").green()
    );
    println!("You can now start using inkspect.");

    Ok(())
}
