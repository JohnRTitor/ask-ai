use clap::{Args, Subcommand};

#[derive(Debug, Args, Clone)]
pub struct BackendArgs {
    #[command(subcommand)]
    pub command: BackendCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum BackendCommand {
    #[command(about = "List known backends")]
    List,
    #[command(about = "Set the active backend")]
    Set { name: String },
    #[command(about = "Show current backend")]
    Show,
}
