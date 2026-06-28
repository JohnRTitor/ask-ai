use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub backend: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    
    #[serde(flatten)]
    pub params: SamplingParams,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            backend: "ollama".to_string(),
            base_url: "http://localhost:11434/v1".to_string(),
            api_key: "none".to_string(),
            model: "".to_string(),
            params: SamplingParams::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SamplingParams {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub min_p: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub mirostat: Option<u8>,
    pub mirostat_tau: Option<f32>,
    pub mirostat_eta: Option<f32>,
    pub seed: Option<u64>,
    pub max_tokens: Option<u32>,
    pub context: Option<u32>,
    pub stream: Option<bool>,
}
