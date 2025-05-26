use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProviderConfig {
    Azure {
        endpoint: String,
        api_key: String,
        engine: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub provider: Option<String>,
    pub azure_endpoint: Option<String>,
    pub azure_api_key: Option<String>,
    pub azure_deployment: Option<String>,
}

impl Config {
    pub fn config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("azure-openai-cli");
        fs::create_dir_all(&path).ok();
        path.push("config.toml");
        path
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Ok(contents) = toml::to_string_pretty(self) {
            let _ = fs::write(path, contents);
        }
    }
}
