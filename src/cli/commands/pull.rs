use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct PullArgs {
    #[arg(help = "Model to pull")]
    pub model: String,
}
