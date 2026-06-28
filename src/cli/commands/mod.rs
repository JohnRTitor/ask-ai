use clap::Subcommand;

pub mod backend;
pub mod chat;
pub mod complete;
pub mod config;
pub mod models;
pub mod ps;
pub mod pull;
pub mod run;
pub mod stop;
pub mod version;

#[derive(Debug, Subcommand, Clone)]
pub enum Command {
    #[command(about = "Single-shot generation")]
    Run(run::RunArgs),

    #[command(about = "Interactive multi-turn chat")]
    Chat(chat::ChatArgs),

    #[command(about = "Raw text completion")]
    Complete(complete::CompleteArgs),

    #[command(about = "List available models")]
    Models(models::ModelsArgs),

    #[command(about = "Pull/download a model")]
    Pull(pull::PullArgs),

    #[command(about = "List running/loaded models")]
    Ps(ps::PsArgs),

    #[command(about = "Unload a model from memory")]
    Stop(stop::StopArgs),

    #[command(about = "Configuration management")]
    Config(config::ConfigArgs),

    #[command(about = "Backend management")]
    Backend(backend::BackendArgs),

    #[command(about = "Print version info")]
    Version,
}
