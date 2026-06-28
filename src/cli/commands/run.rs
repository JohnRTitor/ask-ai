use crate::cli::args::SamplingArgs;
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct RunArgs {
    #[arg(help = "Model to run (e.g., qwen3:8b)")]
    pub model: String,

    #[arg(help = "Prompt text")]
    pub prompt: Option<String>,

    #[command(flatten)]
    pub params: SamplingArgs,
}
