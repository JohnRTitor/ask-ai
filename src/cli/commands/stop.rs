use clap::Args;

#[derive(Debug, Args, Clone)]
pub struct StopArgs {
    #[arg(help = "Model to stop")]
    pub model: Option<String>,
}
