mod provider;
mod config;

use provider::*;
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

fn get_config_value(env_key: &str, config_value: &Option<String>, default: &str) -> String {
    std::env::var(env_key).ok()
        .or_else(|| config_value.clone())
        .unwrap_or_else(|| default.to_string())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Config { action }) => {
            let mut config = Config::load();
            match action {
                Some(ConfigAction::Show) | None => {
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
            }
            return;
        }
        None => {
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
    let provider: Box<dyn LLMProvider + Send + Sync> = match provider_name {
        "azure" => {
            let endpoint = get_config_value("AZURE_OPENAI_ENDPOINT", &config.azure_endpoint, "https://<your-endpoint>.openai.azure.com");
            let api_key = get_config_value("AZURE_OPENAI_API_KEY", &config.azure_api_key, "<your-api-key>");
            let deployment = get_config_value("AZURE_OPENAI_DEPLOYMENT", &config.azure_deployment, "<your-deployment>");
            Box::new(AzureOpenAIProvider { endpoint, api_key, deployment })
        }
        _ => {
            eprintln!("Unsupported provider: {}", provider_name);
            return;
        }
    };
    match provider.chat_with_params(input_string, max_tokens, temperature).await {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}