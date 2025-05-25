mod provider;
mod config;

use std::env;
use provider::*;
use config::Config;

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
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "config" {
        let mut config = Config::load();
        if args.len() == 2 || (args.len() == 3 && args[2] == "show") {
            println!("Current config: {:#?}", config);
            return;
        } else if args.len() == 5 && args[2] == "set" {
            match args[3].as_str() {
                "provider" => config.provider = Some(args[4].clone()),
                "azure_endpoint" => config.azure_endpoint = Some(args[4].clone()),
                "azure_api_key" => config.azure_api_key = Some(args[4].clone()),
                "azure_deployment" => config.azure_deployment = Some(args[4].clone()),
                _ => {
                    print_config_help();
                    return;
                }
            }
            config.save();
            println!("Config updated.");
            return;
        } else {
            print_config_help();
            return;
        }
    }
    // Load config as fallback
    let config = Config::load();
    if args.len() < 3 {
        let provider = config.provider.as_deref().unwrap_or("");
        if provider.is_empty() {
            eprintln!("Usage: {} --provider <provider> <input_string>", args[0]);
            return;
        }
        // Use config defaults if no args
        let input_string = args.get(1).map(|s| s.as_str()).unwrap_or("");
        if input_string.is_empty() {
            eprintln!("No input string provided.");
            return;
        }
        run_with_provider(provider, input_string, &config).await;
        return;
    }
    let provider_name = &args[2];
    let input_string = &args[3];
    run_with_provider(provider_name, input_string, &config).await;
}

async fn run_with_provider(provider_name: &str, input_string: &str, config: &Config) {
    let provider: Box<dyn LLMProvider> = match provider_name {
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
    match provider.chat(input_string).await {
        Ok(response) => println!("{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}