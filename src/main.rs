mod provider;
mod config;

use config::Config;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional maximum number of tokens
    #[arg(long)]
    max_tokens: Option<u32>,

    /// Optional temperature
    #[arg(long)]
    temperature: Option<f32>,

    #[command(subcommand)]
    command: Option<Commands>,

    /// The prompt to send to the LLM (if not using subcommand)
    input: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show or set config
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigAction {
    /// Show current config
    Show,
    /// Set a config value
    Set {
        key: String,
        value: String,
    },
}

fn print_config_help() {
    println!("Usage: azure-openai-cli config [set <key> <value>] [show]");
    println!("Keys: provider, azure_endpoint, azure_api_key, azure_deployment");
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config { action }) => {
            let mut config = Config::load();
            match action {
                Some(ConfigAction::Show) => {
                    println!("Current config: {:#?}", config);
                }
                Some(ConfigAction::Set { key, value }) => {
                    match key.as_str() {
                        "provider" => config.provider = Some(value.clone()),
                        "azure_endpoint" => config.azure_endpoint = Some(value.clone()),
                        "azure_api_key" => config.azure_api_key = Some(value.clone()),
                        "azure_deployment" => config.azure_deployment = Some(value.clone()),
                        _ => {
                            print_config_help();
                            return;
                        }
                    }
                    config.save();
                    println!("Config updated.");
                }
                None => {
                    print_config_help();
                }
            }
            return;
        }
        _ => {
            let config = Config::load();
            let provider = config.provider.as_deref().unwrap_or("azure");
            let input_string = cli.input.as_deref().unwrap_or("");
            if input_string.is_empty() {
                eprintln!("No input string provided.");
                return;
            }
            run_with_provider(
                provider,
                input_string,
                &config,
                cli.max_tokens,
                cli.temperature,
            )
            .await;
        }
    }
}

async fn run_with_provider(provider_name: &str, input_string: &str, config: &Config, max_tokens: Option<u32>, temperature: Option<f32>) {
    let provider = provider::get_provider(provider_name, config);
    let provider = match provider {
        Some(p) => p,
        _ => {
            eprintln!("Unsupported provider: {}", provider_name);
            return;
        }
    };
    match provider.chat_with_params(input_string, max_tokens, temperature).await {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}