use crate::client::types::Usage;
use colored::Colorize;

pub fn print_metrics(usage: Option<Usage>) {
    if let Some(u) = usage {
        println!(
            "\n{}",
            format!(
                "[Tokens: {} prompt, {} completion, {} total]",
                u.prompt_tokens, u.completion_tokens, u.total_tokens
            )
            .dimmed()
            .italic()
        );
    }
}
