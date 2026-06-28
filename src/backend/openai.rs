use crate::backend::Backend;
use crate::client::{ApiClient, Result};
use crate::client::types::Model;
use crate::config::types::SamplingParams;
use async_trait::async_trait;
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
}

#[async_trait]
impl Backend for OpenAIBackend {
    fn name(&self) -> &str {
        "OpenAI"
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn list_models(&self, client: &ApiClient) -> Result<Vec<Model>> {
        let resp = client.list_models().await?;
        Ok(resp.data)
    }

    fn build_extra_body(&self, params: &SamplingParams) -> serde_json::Value {
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
