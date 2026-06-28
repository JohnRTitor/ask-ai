pub mod args;
pub mod commands;

use args::GlobalArgs;
use clap::Parser;
pub use commands::Command;

#[derive(Debug, Parser)]
#[command(name = "ask-ai", about = "Backend-agnostic LLM terminal helper")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[command(flatten)]
    pub global: GlobalArgs,
}
