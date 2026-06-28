use crate::repl::commands::handle_meta_command;
use crate::session::history::ConversationHistory;
use crate::client::{ApiClient, stream::process_stream};
use crate::client::types::{ChatCompletionRequest, ChatMessage};
use crate::backend::Backend;
use crate::config::types::SamplingParams;
use crate::render::{StreamRenderer, spinner::Spinner};
use anyhow::Result;
use colored::Colorize;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use futures_util::StreamExt;
use std::sync::Arc;

pub mod commands;

pub async fn run_repl(
    client: &ApiClient,
    backend: Arc<dyn Backend>,
    model: &str,
    params: &SamplingParams,
) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let history_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ask-ai")
        .join("history.txt");
        
    if let Some(parent) = history_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let _ = rl.load_history(&history_path);

    let mut history = ConversationHistory::new();
    println!("{} {}", "Chatting with".dimmed(), model.cyan().bold());
    println!("{}", "Type /help for commands, Ctrl+C to cancel generation, Ctrl+D to exit".dimmed());

    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                let _ = rl.add_history_entry(line);
                let _ = rl.save_history(&history_path);

                if line.starts_with('/') {
                    let should_exit = handle_meta_command(line, &mut history, &mut rl)?;
                    if should_exit {
                        break;
                    }
                    continue;
                }

                history.add_user_message(line);

                let req = ChatCompletionRequest {
                    model: model.to_string(),
                    messages: history.get_messages(),
                    temperature: params.temperature,
                    top_p: params.top_p,
                    stream: Some(true),
                    max_tokens: params.max_tokens,
                    extra: backend.build_extra_body(params),
                };

                let spinner = Spinner::new("Thinking...");
                let source_res = client.chat_completion_stream(&req).await;
                spinner.finish_and_clear();

                match source_res {
                    Ok(source) => {
                        let mut stream = process_stream(source);
                        let mut renderer = StreamRenderer::new();
                        let mut full_response = String::new();

                        while let Some(Ok(event)) = stream.next().await {
                            renderer.append(&event.content);
                            full_response.push_str(&event.content);
                        }
                        
                        renderer.finish();
                        history.add_assistant_message(&full_response);
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error:".red(), e);
                        // Remove user message from history on failure so they can try again
                        if !history.messages.is_empty() {
                            history.messages.pop();
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C at prompt
                println!("Interrupted (Ctrl-D to exit)");
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
