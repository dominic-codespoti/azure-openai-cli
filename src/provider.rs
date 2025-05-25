use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider {
    async fn chat(&self, prompt: &str) -> Result<String, String>;
}

pub struct AzureOpenAIProvider {
    pub endpoint: String,
    pub api_key: String,
    pub deployment: String,
}

#[async_trait]
impl LLMProvider for AzureOpenAIProvider {
    async fn chat(&self, prompt: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview",
            self.endpoint.trim_end_matches('/'), self.deployment
        );
        let payload = serde_json::json!({
            "messages": [
                { "role": "user", "content": prompt }
            ],
            "max_tokens": 50,
            "temperature": 0.7,
        });
        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("api-key", &self.api_key)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        let status = resp.status();
        if status.is_success() {
            let body: serde_json::Value = resp.json().await.map_err(|e| format!("Invalid JSON: {}", e))?;
            Ok(body["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
        } else {
            let err_body = resp.text().await.unwrap_or_default();
            Err(format!("Error: {} - {}", status, err_body))
        }
    }
}
