use crate::session::history::ConversationHistory;
use anyhow::Result;
use colored::Colorize;
use rustyline::DefaultEditor;

pub fn handle_meta_command(
    line: &str,
    history: &mut ConversationHistory,
    _rl: &mut DefaultEditor,
) -> Result<bool> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let cmd = parts[0];

    match cmd {
        "/help" => {
            println!("{}", "Available Commands:".bold());
            println!("  /help     - Show this help message");
            println!("  /clear    - Clear conversation history");
            println!("  /system   - Set system prompt (e.g. /system You are a helpful assistant)");
            println!("  /exit     - Exit the REPL");
            println!("  /quit     - Exit the REPL");
            Ok(false)
        }
        "/clear" => {
            history.clear();
            println!("{}", "Conversation history cleared.".green());
            Ok(false)
        }
        "/system" => {
            let prompt = line.strip_prefix("/system").unwrap_or("").trim();
            if prompt.is_empty() {
                println!("{}", "Error: must provide a system prompt.".red());
            } else {
                history.set_system_prompt(prompt);
                println!("{}", "System prompt updated.".green());
            }
            Ok(false)
        }
        "/exit" | "/quit" => Ok(true),
        _ => {
            println!(
                "{} Type /help for a list of commands.",
                format!("Unknown command: {}", cmd).red()
            );
            Ok(false)
        }
    }
}
