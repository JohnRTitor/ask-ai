use crate::backend::Backend;
use crate::client::{ApiClient, Result};
use crate::client::error::ClientError;
use crate::client::types::{Model, OllamaPsResponse, OllamaRunningModel};
use crate::config::types::SamplingParams;
use async_trait::async_trait;
use serde_json::json;

pub struct OllamaBackend {
    base_url: String,
}

impl OllamaBackend {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    // Convert /v1 base URL to native /api base URL for Ollama-specific endpoints
    fn native_api_url(&self) -> String {
        self.base_url.replace("/v1", "/api")
    }
}

#[async_trait]
impl Backend for OllamaBackend {
    fn name(&self) -> &str {
        "Ollama"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn supports_pull(&self) -> bool { true }
    fn supports_ps(&self) -> bool { true }
    fn supports_stop(&self) -> bool { true }

    async fn list_models(&self, client: &ApiClient) -> Result<Vec<Model>> {
        // We can just use the OpenAI-compatible /v1/models endpoint
        let resp = client.list_models().await?;
        Ok(resp.data)
    }

    async fn pull_model(&self, client: &ApiClient, model: &str) -> Result<()> {
        let url = format!("{}/pull", self.native_api_url());
        
        let resp = client.client.post(&url)
            .json(&json!({
                "name": model,
                "stream": false
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api(format!("Failed to pull model: {}", err_text)));
        }

        Ok(())
    }

    async fn list_running(&self, client: &ApiClient) -> Result<Vec<OllamaRunningModel>> {
        let url = format!("{}/ps", self.native_api_url());
        
        let resp = client.client.get(&url).send().await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api(format!("Failed to list running models: {}", err_text)));
        }

        let data = resp.json::<OllamaPsResponse>().await?;
        Ok(data.models)
    }

    async fn stop_model(&self, client: &ApiClient, model: &str) -> Result<()> {
        // In Ollama, sending keep_alive: 0 unloads the model
        let url = format!("{}/generate", self.native_api_url());
        
        let resp = client.client.post(&url)
            .json(&json!({
                "model": model,
                "keep_alive": 0
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(ClientError::Api(format!("Failed to stop model: {}", err_text)));
        }

        Ok(())
    }

    fn build_extra_body(&self, params: &SamplingParams) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        
        // Options for Ollama must be placed inside an "options" object
        let mut options = serde_json::Map::new();
        
        if let Some(v) = params.temperature { options.insert("temperature".to_string(), json!(v)); }
        if let Some(v) = params.top_p { options.insert("top_p".to_string(), json!(v)); }
        if let Some(v) = params.top_k { options.insert("top_k".to_string(), json!(v)); }
        if let Some(v) = params.min_p { options.insert("min_p".to_string(), json!(v)); }
        if let Some(v) = params.repeat_penalty { options.insert("repeat_penalty".to_string(), json!(v)); }
        if let Some(v) = params.presence_penalty { options.insert("presence_penalty".to_string(), json!(v)); }
        if let Some(v) = params.frequency_penalty { options.insert("frequency_penalty".to_string(), json!(v)); }
        if let Some(v) = params.mirostat { options.insert("mirostat".to_string(), json!(v)); }
        if let Some(v) = params.mirostat_tau { options.insert("mirostat_tau".to_string(), json!(v)); }
        if let Some(v) = params.mirostat_eta { options.insert("mirostat_eta".to_string(), json!(v)); }
        if let Some(v) = params.seed { options.insert("seed".to_string(), json!(v)); }
        if let Some(v) = params.context { options.insert("num_ctx".to_string(), json!(v)); }
        
        if !options.is_empty() {
            map.insert("options".to_string(), serde_json::Value::Object(options));
        }
        
        serde_json::Value::Object(map)
    }
}
