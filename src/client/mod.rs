pub mod error;
pub mod stream;
pub mod types;

use reqwest::Client;
use types::{ChatCompletionRequest, ChatMessage, ModelListResponse, OllamaPsResponse};
pub use error::{ClientError, Result};
use std::time::Duration;
use reqwest_eventsource::{EventSource, RequestBuilderExt};

#[derive(Clone)]
pub struct ApiClient {
    pub client: Client,
    pub base_url: String,
    pub api_key: String,
}

impl ApiClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub async fn chat_completion_stream(&self, req: &ChatCompletionRequest) -> Result<EventSource> {
        let url = self.build_url("/chat/completions");
        
        let mut request = self.client.post(&url)
            .json(req);

        if !self.api_key.is_empty() && self.api_key != "none" {
            request = request.bearer_auth(&self.api_key);
        }

        let source = request.eventsource().map_err(|e| ClientError::Api(e.to_string()))?;
        Ok(source)
    }

    pub async fn list_models(&self) -> Result<ModelListResponse> {
        let url = self.build_url("/models");
        
        let mut request = self.client.get(&url);
        if !self.api_key.is_empty() && self.api_key != "none" {
            request = request.bearer_auth(&self.api_key);
        }

        let resp = request.send().await?;
        if !resp.status().is_success() {
            return Err(ClientError::Api(format!("Failed to list models: {}", resp.status())));
        }

        let data = resp.json::<ModelListResponse>().await?;
        Ok(data)
    }
}
