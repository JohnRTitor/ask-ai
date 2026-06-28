use crate::client::{ApiClient, Result};
use crate::client::types::{Model, OllamaRunningModel};
use crate::config::types::SamplingParams;
use async_trait::async_trait;

pub mod openai;
pub mod ollama;
pub mod llamacpp;

#[async_trait]
pub trait Backend: Send + Sync {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    
    fn supports_pull(&self) -> bool { false }
    fn supports_ps(&self) -> bool { false }
    fn supports_stop(&self) -> bool { false }
    
    async fn list_models(&self, client: &ApiClient) -> Result<Vec<Model>>;
    
    async fn pull_model(&self, _client: &ApiClient, _model: &str) -> Result<()> {
        Err(crate::client::error::ClientError::Api("Backend does not support pull".to_string()))
    }
    
    async fn list_running(&self, _client: &ApiClient) -> Result<Vec<OllamaRunningModel>> {
        Err(crate::client::error::ClientError::Api("Backend does not support ps".to_string()))
    }
    
    async fn stop_model(&self, _client: &ApiClient, _model: &str) -> Result<()> {
        Err(crate::client::error::ClientError::Api("Backend does not support stop".to_string()))
    }
    
    fn build_extra_body(&self, params: &SamplingParams) -> serde_json::Value;
}

pub fn create_backend(name: &str, base_url: &str) -> Box<dyn Backend> {
    match name.to_lowercase().as_str() {
        "ollama" => Box::new(ollama::OllamaBackend::new(base_url)),
        "llamacpp" | "llama.cpp" => Box::new(llamacpp::LlamaCppBackend::new(base_url)),
        _ => Box::new(openai::OpenAIBackend::new(base_url)),
    }
}
