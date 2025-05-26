use async_trait::async_trait;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, prompt: &str) -> Result<String, String>;
    async fn chat_with_params(&self, prompt: &str, _max_tokens: Option<u32>, _temperature: Option<f32>) -> Result<String, String> {
        self.chat(prompt).await
    }
}

pub struct AzureOpenAIProvider {
    pub endpoint: String,
    pub api_key: String,
    pub deployment: String,
}

#[async_trait]
impl LLMProvider for AzureOpenAIProvider {
    async fn chat(&self, prompt: &str) -> Result<String, String> {
        self.chat_with_params(prompt, None, None).await
    }

    async fn chat_with_params(&self, prompt: &str, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<String, String> {
        let client = reqwest::Client::new();
        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview",
            self.endpoint.trim_end_matches('/'), self.deployment
        );
        let mut payload = serde_json::json!({
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });
        if let Some(mt) = max_tokens { payload["max_tokens"] = serde_json::json!(mt); }
        else { payload["max_tokens"] = serde_json::json!(50); }
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
