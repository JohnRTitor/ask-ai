use clap::Args;
use crate::cli::args::SamplingArgs;

#[derive(Debug, Args, Clone)]
pub struct ChatArgs {
    #[arg(help = "Model to run (e.g., qwen3:8b)")]
    pub model: Option<String>,

    #[command(flatten)]
    pub params: SamplingArgs,
}
