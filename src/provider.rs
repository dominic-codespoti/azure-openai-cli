use async_trait::async_trait;

pub mod azure;

#[async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat_with_params(&self, prompt: &str, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<String, String>;
}

pub fn get_provider(name: &str, config: &crate::Config) -> Option<Box<dyn LLMProvider + Send + Sync>> {
    match name.to_lowercase().as_str() {
        "azure" => azure::AzureOpenAIProvider::from_config(config),
        _ => None,
    }
}
