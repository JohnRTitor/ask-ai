use crate::backend::Backend;
use crate::client::{ApiClient, Result};
use crate::client::types::Model;
use crate::config::types::SamplingParams;
use async_trait::async_trait;
use serde_json::json;

pub struct LlamaCppBackend {
    base_url: String,
}

impl LlamaCppBackend {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl Backend for LlamaCppBackend {
    fn name(&self) -> &str {
        "llama.cpp"
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
        
        // llama.cpp supports many params at the root level of the request
        if let Some(v) = params.min_p { map.insert("min_p".to_string(), json!(v)); }
        if let Some(v) = params.repeat_penalty { map.insert("repeat_penalty".to_string(), json!(v)); }
        if let Some(v) = params.presence_penalty { map.insert("presence_penalty".to_string(), json!(v)); }
        if let Some(v) = params.frequency_penalty { map.insert("frequency_penalty".to_string(), json!(v)); }
        if let Some(v) = params.mirostat { map.insert("mirostat".to_string(), json!(v)); }
        if let Some(v) = params.mirostat_tau { map.insert("mirostat_tau".to_string(), json!(v)); }
        if let Some(v) = params.mirostat_eta { map.insert("mirostat_eta".to_string(), json!(v)); }
        if let Some(v) = params.seed { map.insert("seed".to_string(), json!(v)); }
        // For llama.cpp n_predict is max_tokens, but usually max_tokens is translated automatically
        // However, we'll let the standard OpenAI struct handle max_tokens.
        
        serde_json::Value::Object(map)
    }
}
