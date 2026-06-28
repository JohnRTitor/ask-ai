use crate::cli::args::SamplingArgs;
use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct ChatArgs {
    #[arg(help = "Model to run (e.g., qwen3:8b)")]
    pub model: Option<String>,

    #[command(flatten)]
    pub params: SamplingArgs,
}
