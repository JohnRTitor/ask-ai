use crate::client::error::{ClientError, Result};
use crate::client::types::{ChatCompletionChunk, Usage};
use eventsource_stream::Eventsource;
use futures_util::StreamExt;
use futures_util::stream::Stream;
use reqwest::Response;
use std::pin::Pin;

pub struct StreamEvent {
    pub content: String,
    pub usage: Option<Usage>,
}

pub fn process_stream(
    response: Response,
) -> Pin<Box<dyn Stream<Item = Result<StreamEvent>> + Send>> {
    let mut source = response.bytes_stream().eventsource();

    let stream = async_stream::stream! {
        while let Some(event) = source.next().await {
            match event {
                Ok(message) => {
                    if message.data == "[DONE]" {
                        break;
                    }

                    match serde_json::from_str::<ChatCompletionChunk>(&message.data) {
                        Ok(chunk) => {
                            let content = chunk.choices.first()
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
                Err(e) => {
                    yield Err(ClientError::StreamError(e.to_string()));
                    break;
                }
            }
        }
    };

    Box::pin(stream)
}
