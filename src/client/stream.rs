use reqwest_eventsource::{Event, EventSource};
use futures_util::StreamExt;
use crate::client::error::{ClientError, Result};
use crate::client::types::{ChatCompletionChunk, Usage};
use std::pin::Pin;
use futures_util::stream::Stream;

pub struct StreamEvent {
    pub content: String,
    pub usage: Option<Usage>,
}

pub fn process_stream(
    mut source: EventSource,
) -> Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>> {
    let stream = async_stream::stream! {
        while let Some(event) = source.next().await {
            match event {
                Ok(Event::Open) => {}
                Ok(Event::Message(message)) => {
                    if message.data == "[DONE]" {
                        source.close();
                        break;
                    }
                    
                    match serde_json::from_str::<ChatCompletionChunk>(&message.data) {
                        Ok(chunk) => {
                            let content = chunk.choices.get(0)
                                .and_then(|c| c.delta.content.clone())
                                .unwrap_or_default();
                            
                            yield Ok(StreamEvent {
                                content,
                                usage: chunk.usage,
                            });
                        }
                        Err(e) => {
                            yield Err(ClientError::ParseError(e));
                        }
                    }
                }
                Err(reqwest_eventsource::Error::StreamEnded) => {
                    break;
                }
                Err(e) => {
                    yield Err(ClientError::StreamError(e.to_string()));
                    break;
                }
            }
        }
    };
    
    Box::pin(stream)
}
