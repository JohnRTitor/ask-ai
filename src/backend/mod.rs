use crate::client::types::{Model, OllamaRunningModel};
use crate::client::{ApiClient, Result};
use crate::config::types::SamplingParams;

pub mod llamacpp;
pub mod ollama;
pub mod openai;

pub enum Backend {
    OpenAI(openai::OpenAIBackend),
    Ollama(ollama::OllamaBackend),
    LlamaCpp(llamacpp::LlamaCppBackend),
}

impl Backend {
    pub fn name(&self) -> &str {
        match self {
            Self::OpenAI(b) => b.name(),
            Self::Ollama(b) => b.name(),
            Self::LlamaCpp(b) => b.name(),
        }
    }

    pub fn base_url(&self) -> &str {
        match self {
            Self::OpenAI(b) => b.base_url(),
            Self::Ollama(b) => b.base_url(),
            Self::LlamaCpp(b) => b.base_url(),
        }
    }

    pub fn supports_pull(&self) -> bool {
        match self {
            Self::OpenAI(b) => b.supports_pull(),
            Self::Ollama(b) => b.supports_pull(),
            Self::LlamaCpp(b) => b.supports_pull(),
        }
    }

    pub fn supports_ps(&self) -> bool {
        match self {
            Self::OpenAI(b) => b.supports_ps(),
            Self::Ollama(b) => b.supports_ps(),
            Self::LlamaCpp(b) => b.supports_ps(),
        }
    }

    pub fn supports_stop(&self) -> bool {
        match self {
            Self::OpenAI(b) => b.supports_stop(),
            Self::Ollama(b) => b.supports_stop(),
            Self::LlamaCpp(b) => b.supports_stop(),
        }
    }

    pub async fn list_models(&self, client: &ApiClient) -> Result<Vec<Model>> {
        match self {
            Self::OpenAI(b) => b.list_models(client).await,
            Self::Ollama(b) => b.list_models(client).await,
            Self::LlamaCpp(b) => b.list_models(client).await,
        }
    }

    pub async fn pull_model(&self, client: &ApiClient, model: &str) -> Result<()> {
        match self {
            Self::OpenAI(b) => b.pull_model(client, model).await,
            Self::Ollama(b) => b.pull_model(client, model).await,
            Self::LlamaCpp(b) => b.pull_model(client, model).await,
        }
    }

    pub async fn list_running(&self, client: &ApiClient) -> Result<Vec<OllamaRunningModel>> {
        match self {
            Self::OpenAI(b) => b.list_running(client).await,
            Self::Ollama(b) => b.list_running(client).await,
            Self::LlamaCpp(b) => b.list_running(client).await,
        }
    }

    pub async fn stop_model(&self, client: &ApiClient, model: &str) -> Result<()> {
        match self {
            Self::OpenAI(b) => b.stop_model(client, model).await,
            Self::Ollama(b) => b.stop_model(client, model).await,
            Self::LlamaCpp(b) => b.stop_model(client, model).await,
        }
    }

    pub fn build_extra_body(&self, params: &SamplingParams) -> serde_json::Value {
        match self {
            Self::OpenAI(b) => b.build_extra_body(params),
            Self::Ollama(b) => b.build_extra_body(params),
            Self::LlamaCpp(b) => b.build_extra_body(params),
        }
    }
}

pub fn create_backend(name: &str, base_url: &str) -> Backend {
    match name.to_lowercase().as_str() {
        "ollama" => Backend::Ollama(ollama::OllamaBackend::new(base_url)),
        "llamacpp" | "llama.cpp" => Backend::LlamaCpp(llamacpp::LlamaCppBackend::new(base_url)),
        _ => Backend::OpenAI(openai::OpenAIBackend::new(base_url)),
    }
}
