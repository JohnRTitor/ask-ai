use crate::client::types::{Model, OllamaRunningModel};
use crate::client::{ApiClient, Result};
use crate::config::types::SamplingParams;
use serde_json::json;

pub struct OpenAIBackend {
    base_url: String,
}

impl OpenAIBackend {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        "OpenAI"
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn supports_pull(&self) -> bool {
        false
    }
    pub fn supports_ps(&self) -> bool {
        false
    }
    pub fn supports_stop(&self) -> bool {
        false
    }

    pub async fn list_models(&self, client: &ApiClient) -> Result<Vec<Model>> {
        let resp = client.list_models().await?;
        Ok(resp.data)
    }

    pub async fn pull_model(&self, _client: &ApiClient, _model: &str) -> Result<()> {
        Err(crate::client::error::ClientError::Api(
            "Backend does not support pull".to_string(),
        ))
    }

    pub async fn list_running(&self, _client: &ApiClient) -> Result<Vec<OllamaRunningModel>> {
        Err(crate::client::error::ClientError::Api(
            "Backend does not support ps".to_string(),
        ))
    }

    pub async fn stop_model(&self, _client: &ApiClient, _model: &str) -> Result<()> {
        Err(crate::client::error::ClientError::Api(
            "Backend does not support stop".to_string(),
        ))
    }

    pub fn build_extra_body(&self, params: &SamplingParams) -> serde_json::Value {
        let mut map = serde_json::Map::new();

        if let Some(v) = params.presence_penalty {
            map.insert("presence_penalty".to_string(), json!(v));
        }
        if let Some(v) = params.frequency_penalty {
            map.insert("frequency_penalty".to_string(), json!(v));
        }
        if let Some(v) = params.seed {
            map.insert("seed".to_string(), json!(v));
        }

        serde_json::Value::Object(map)
    }
}
