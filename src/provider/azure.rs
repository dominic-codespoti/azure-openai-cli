use async_trait::async_trait;
use crate::provider::LLMProvider;

pub struct AzureOpenAIProvider {
    pub endpoint: String,
    pub api_key: String,
    pub deployment: String,
}

impl AzureOpenAIProvider {
    pub fn from_config(config: &crate::Config) -> Option<Box<dyn crate::provider::LLMProvider + Send + Sync>> {
        Some(Box::new(Self {
            endpoint: config.azure_endpoint.clone()?,
            api_key: config.azure_api_key.clone()?,
            deployment: config.azure_deployment.clone()?,
        }))
    }
}

#[async_trait]
impl LLMProvider for AzureOpenAIProvider {
    async fn chat_with_params(&self, prompt: &str, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview",
            self.endpoint.trim_end_matches('/'), self.deployment
        );
        let mut payload = serde_json::json!({
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "stream": true
        });
        if let Some(mt) = max_tokens { payload["max_tokens"] = serde_json::json!(mt); }
        else { payload["max_tokens"] = serde_json::json!(256); }
        if let Some(temp) = temperature { payload["temperature"] = serde_json::json!(temp); }
        else { payload["temperature"] = serde_json::json!(0.7); }
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("api-key", &self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let stream = resp.bytes().await.map_err(|e| format!("Stream error: {}", e))?;
        let text = String::from_utf8_lossy(&stream);
        use std::io::{self, Write};
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let mut full = String::new();
        let mut printed = false;
        for line in text.split('\n') {
            let line = line.trim_start();
            if line.starts_with("data: ") {
                let json = &line[6..];
                if json == "[DONE]" { break; }
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(json) {
                    if let Some(content) = val["choices"][0]["delta"]["content"].as_str() {
                        write!(handle, "{}", content).ok();
                        handle.flush().ok();
                        full.push_str(content);
                        printed = true;
                    }
                }
            }
        }
        if printed {
            writeln!(handle).ok();
            handle.flush().ok();
        }
        Ok(full)
    }
}
