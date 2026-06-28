use clap::Parser;
use ask_ai::cli::{Cli, Command};
use ask_ai::config;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load config from file
    let mut app_config = config::load_config()?;
    
    // Merge global args
    if let Some(b) = cli.global.backend {
        app_config.backend = b;
    }
    if let Some(url) = cli.global.base_url {
        app_config.base_url = url;
    }
    if let Some(key) = cli.global.api_key {
        app_config.api_key = key;
    }
    
    let client = ask_ai::client::ApiClient::new(&app_config.base_url, &app_config.api_key);
    let backend = ask_ai::backend::create_backend(&app_config.backend, &app_config.base_url);
    let backend_arc: std::sync::Arc<dyn ask_ai::backend::Backend> = std::sync::Arc::from(backend);

    match cli.command {
        Command::Run(args) => {
            let model = if args.model.is_empty() { app_config.model.clone() } else { args.model };
            let params = args.params.merge_into_params(app_config.params.clone());
            
            // Single-shot run
            let prompt = if let Some(p) = args.prompt {
                p
            } else {
                use std::io::Read;
                let mut p = String::new();
                std::io::stdin().read_to_string(&mut p)?;
                p
            };
            
            let req = ask_ai::client::types::ChatCompletionRequest {
                model: model.clone(),
                messages: vec![ask_ai::client::types::ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                }],
                temperature: params.temperature,
                top_p: params.top_p,
                stream: params.stream, // Actually we always stream if not set to false
                max_tokens: params.max_tokens,
                extra: backend_arc.build_extra_body(&params),
            };
            
            let source = client.chat_completion_stream(&req).await?;
            let mut stream = ask_ai::client::stream::process_stream(source);
            let mut renderer = ask_ai::render::StreamRenderer::new();
            
            use futures_util::StreamExt;
            while let Some(Ok(event)) = stream.next().await {
                renderer.append(&event.content);
                if let Some(usage) = event.usage {
                    ask_ai::utils::metrics::print_metrics(Some(usage));
                }
            }
            renderer.finish();
        },
        Command::Chat(args) => {
            let model = args.model.unwrap_or(app_config.model.clone());
            let params = args.params.merge_into_params(app_config.params.clone());
            ask_ai::repl::run_repl(&client, backend_arc, &model, &params).await?;
        },
        Command::Models(_) => {
            match backend_arc.list_models(&client).await {
                Ok(models) => {
                    for m in models {
                        println!("- {}", m.id);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Command::Pull(args) => {
            if backend_arc.supports_pull() {
                println!("Pulling {}...", args.model);
                match backend_arc.pull_model(&client, &args.model).await {
                    Ok(_) => println!("Success!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                eprintln!("Backend does not support pulling models.");
            }
        },
        Command::Ps(_) => {
            if backend_arc.supports_ps() {
                match backend_arc.list_running(&client).await {
                    Ok(models) => {
                        for m in models {
                            println!("- {} ({} bytes)", m.name, m.size);
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                eprintln!("Backend does not support listing running models.");
            }
        },
        Command::Stop(args) => {
            if backend_arc.supports_stop() {
                let model = args.model.unwrap_or(app_config.model.clone());
                println!("Stopping {}...", model);
                match backend_arc.stop_model(&client, &model).await {
                    Ok(_) => println!("Success!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                eprintln!("Backend does not support stopping models.");
            }
        },
        Command::Complete(_) => println!("Not implemented yet"),
        Command::Config(_) => println!("Config: {:?}", app_config),
        Command::Backend(_) => println!("Backend: {}", app_config.backend),
        Command::Version => {
            println!("ask-ai {}", env!("CARGO_PKG_VERSION"));
        }
    }
    
    Ok(())
}
