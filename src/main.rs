use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

mod chat;
mod config;

use chat::ChatClient;
use config::{Config, Provider};

#[derive(Parser)]
#[command(name = "ai-chat")]
#[command(about = "A simple AI chat CLI with configurable providers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure API keys and models
    Config,
    /// Start a chat session
    Chat {
        /// Provider to use (openai, anthropic, etc.)
        #[arg(short, long)]
        provider: Option<String>,
        /// Model to use
        #[arg(short, long)]
        model: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config => configure().await?,
        Commands::Chat { provider, model } => chat(provider, model).await?,
    }

    Ok(())
}

async fn configure() -> Result<()> {
    let mut config = Config::load().unwrap_or_default();

    println!("🔧 AI Chat Configuration");

    let providers = vec!["OpenAI", "Anthropic"];
    let selection = Select::new()
        .with_prompt("Select provider to configure")
        .items(&providers)
        .interact()?;

    let provider_name = providers[selection].to_lowercase();

    let api_key: String = Input::new()
        .with_prompt(&format!("Enter {} API key", providers[selection]))
        .interact_text()?;

    let models = match provider_name.as_str() {
        "openai" => vec!["gpt-4", "gpt-4-turbo", "gpt-3.5-turbo"],
        "anthropic" => vec![
            "claude-3-opus-20240229",
            "claude-3-sonnet-20240229",
            "claude-3-haiku-20240307",
        ],
        _ => vec!["default"],
    };

    let model_selection = Select::new()
        .with_prompt("Select default model")
        .items(&models)
        .interact()?;

    let provider = Provider {
        api_key,
        default_model: models[model_selection].to_string(),
        models: models.iter().map(|s| s.to_string()).collect(),
    };

    config.providers.insert(provider_name, provider);
    config.save()?;

    println!("✅ Configuration saved!");

    Ok(())
}

async fn chat(provider: Option<String>, model: Option<String>) -> Result<()> {
    let config = Config::load()?;

    let provider_name = if let Some(p) = provider {
        p
    } else {
        let provider_names: Vec<String> = config.providers.keys().cloned().collect();
        if provider_names.is_empty() {
            anyhow::bail!("No providers configured. Run 'ai-chat config' first.");
        }

        let selection = Select::new()
            .with_prompt("Select provider")
            .items(&provider_names)
            .interact()?;

        provider_names[selection].clone()
    };

    let provider_config = config
        .providers
        .get(&provider_name)
        .ok_or_else(|| anyhow::anyhow!("Provider '{}' not configured", provider_name))?;

    let model_name = if let Some(m) = model {
        m
    } else {
        let selection = Select::new()
            .with_prompt("Select model")
            .items(&provider_config.models)
            .interact()?;

        provider_config.models[selection].clone()
    };

    let client = ChatClient::new(&provider_name, &provider_config.api_key, &model_name);

    println!(
        "🤖 AI Chat started with {} using {}",
        provider_name, model_name
    );
    println!("Type 'quit' or 'exit' to end the conversation\n");

    loop {
        let user_input: String = Input::new().with_prompt("You").interact_text()?;

        if user_input.trim().to_lowercase() == "quit" || user_input.trim().to_lowercase() == "exit"
        {
            println!("👋 Goodbye!");
            break;
        }

        print!("AI: ");
        match client.send_message(&user_input).await {
            Ok(response) => println!("{}\n", response),
            Err(e) => println!("Error: {}\n", e),
        }
    }

    Ok(())
}
