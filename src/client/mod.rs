pub mod error;
pub mod stream;
pub mod types;

pub use error::{ClientError, Result};
use reqwest::Client;
use reqwest::Response;
use std::time::Duration;
use types::{ChatCompletionRequest, ModelListResponse};

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

    pub async fn chat_completion_stream(&self, req: &ChatCompletionRequest) -> Result<Response> {
        let url = self.build_url("/chat/completions");

        let mut request = self.client.post(&url).json(req);

        if !self.api_key.is_empty() && self.api_key != "none" {
            request = request.bearer_auth(&self.api_key);
        }

        let response = request
            .send()
            .await
            .map_err(|e| ClientError::Api(e.to_string()))?;
        if !response.status().is_success() {
            let status = response.status();
            let err_text = response.text().await.unwrap_or_default();
            return Err(ClientError::Api(format!(
                "API error {}: {}",
                status, err_text
            )));
        }
        Ok(response)
    }

    pub async fn list_models(&self) -> Result<ModelListResponse> {
        let url = self.build_url("/models");

        let mut request = self.client.get(&url);
        if !self.api_key.is_empty() && self.api_key != "none" {
            request = request.bearer_auth(&self.api_key);
        }

        let resp = request.send().await?;
        if !resp.status().is_success() {
            return Err(ClientError::Api(format!(
                "Failed to list models: {}",
                resp.status()
            )));
        }

        let data = resp.json::<ModelListResponse>().await?;
        Ok(data)
    }
}
