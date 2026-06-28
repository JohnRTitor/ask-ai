use clap::{Args, Subcommand};

#[derive(Debug, Args, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Debug, Subcommand, Clone)]
pub enum ConfigCommand {
    #[command(about = "Show current configuration")]
    Show,
    #[command(about = "Set a configuration value")]
    Set { key: String, value: String },
    #[command(about = "Show configuration file path")]
    Path,
}
