use clap::{Args, Parser};

#[derive(Debug, Args, Clone)]
pub struct GlobalArgs {
    #[arg(long, global = true, help = "Override config backend (e.g. ollama, openai, llamacpp)")]
    pub backend: Option<String>,

    #[arg(long, global = true, help = "Override base URL for the API")]
    pub base_url: Option<String>,

    #[arg(long, global = true, help = "Override API key")]
    pub api_key: Option<String>,
}

#[derive(Debug, Args, Clone, Default)]
pub struct SamplingArgs {
    #[arg(long, help = "Sampling temperature (e.g., 0.7)")]
    pub temperature: Option<f32>,

    #[arg(long, help = "Top-p sampling (e.g., 0.9)")]
    pub top_p: Option<f32>,

    #[arg(long, help = "Top-k sampling (e.g., 40)")]
    pub top_k: Option<u32>,

    #[arg(long, help = "Min-p sampling (e.g., 0.05)")]
    pub min_p: Option<f32>,

    #[arg(long, help = "Repeat penalty (e.g., 1.1)")]
    pub repeat_penalty: Option<f32>,

    #[arg(long, help = "Presence penalty (e.g., 0.0)")]
    pub presence_penalty: Option<f32>,

    #[arg(long, help = "Frequency penalty (e.g., 0.0)")]
    pub frequency_penalty: Option<f32>,

    #[arg(long, help = "Mirostat sampling (0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0)")]
    pub mirostat: Option<u8>,

    #[arg(long, help = "Mirostat target entropy (tau) (e.g., 5.0)")]
    pub mirostat_tau: Option<f32>,

    #[arg(long, help = "Mirostat learning rate (eta) (e.g., 0.1)")]
    pub mirostat_eta: Option<f32>,

    #[arg(long, help = "Random seed")]
    pub seed: Option<u64>,

    #[arg(long, help = "Max tokens to generate")]
    pub max_tokens: Option<u32>,

    #[arg(long, help = "Context window size")]
    pub context: Option<u32>,

    #[arg(long, help = "Stream the response", action = clap::ArgAction::SetTrue)]
    pub stream: bool,

    #[arg(long, help = "Do not stream the response", action = clap::ArgAction::SetTrue)]
    pub no_stream: bool,
}

impl SamplingArgs {
    pub fn merge_into_params(self, mut params: crate::config::types::SamplingParams) -> crate::config::types::SamplingParams {
        if let Some(v) = self.temperature { params.temperature = Some(v); }
        if let Some(v) = self.top_p { params.top_p = Some(v); }
        if let Some(v) = self.top_k { params.top_k = Some(v); }
        if let Some(v) = self.min_p { params.min_p = Some(v); }
        if let Some(v) = self.repeat_penalty { params.repeat_penalty = Some(v); }
        if let Some(v) = self.presence_penalty { params.presence_penalty = Some(v); }
        if let Some(v) = self.frequency_penalty { params.frequency_penalty = Some(v); }
        if let Some(v) = self.mirostat { params.mirostat = Some(v); }
        if let Some(v) = self.mirostat_tau { params.mirostat_tau = Some(v); }
        if let Some(v) = self.mirostat_eta { params.mirostat_eta = Some(v); }
        if let Some(v) = self.seed { params.seed = Some(v); }
        if let Some(v) = self.max_tokens { params.max_tokens = Some(v); }
        if let Some(v) = self.context { params.context = Some(v); }
        
        if self.stream {
            params.stream = Some(true);
        } else if self.no_stream {
            params.stream = Some(false);
        }
        
        params
    }
}
