use crate::client::IFlowClient;
use crate::error::Result;
use crate::types::{Message, IFlowOptions};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

/// Simple synchronous query to iFlow
///
/// Sends a query to iFlow and waits for a complete response.
/// This is a convenience function for simple request-response interactions.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
///
/// # Returns
/// * `Ok(String)` containing the response from iFlow
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::query;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let response = query("What is 2 + 2?").await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
pub async fn query(prompt: &str) -> Result<String> {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            // Create client with a reasonable timeout
            let options = IFlowOptions::new().with_timeout(30.0);
            let mut client = IFlowClient::new(Some(options));
            client.connect().await?;

            client.send_message(prompt, None).await?;

            let mut response = String::new();
            let mut message_stream = client.messages();

            // 使用带超时的循环接收消息
            let mut finished = false;
            while !finished {
                match timeout(Duration::from_secs(30), message_stream.next()).await {
                    Ok(Some(message)) => {
                        match message {
                            Message::Assistant { content } => {
                                response.push_str(&content);
                            }
                            Message::TaskFinish { .. } => {
                                finished = true;
                            }
                            _ => {}
                        }
                    }
                    Ok(None) => {
                        // 流结束
                        finished = true;
                    }
                    Err(_) => {
                        // 超时
                        finished = true;
                    }
                }
            }

            client.disconnect().await?;
            Ok(response.trim().to_string())
        })
        .await
}

/// Stream responses from iFlow
///
/// Sends a query to iFlow and returns a stream of response chunks.
/// This is useful for real-time output as the response is generated.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
///
/// # Returns
/// * `Ok(impl Stream<Item = String>)` containing the response stream
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::query_stream;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut stream = query_stream("Tell me a story").await?;
///     
///     while let Some(chunk) = stream.next().await {
///         print!("{}", chunk);
///         // Flush stdout for real-time output
///         use std::io::{self, Write};
///         io::stdout().flush()?;
///     }
///     
///     Ok(())
/// }
/// ```
pub async fn query_stream(prompt: &str) -> Result<impl futures::Stream<Item = String>> {
    let local = tokio::task::LocalSet::new();
    // We need to run this in a LocalSet context but return a stream
    // Let's create the client and connection in the LocalSet context
    local
        .run_until(async {
            // Create client with a reasonable timeout
            let options = IFlowOptions::new().with_timeout(30.0);
            let mut client = IFlowClient::new(Some(options));
            client.connect().await?;

            client.send_message(prompt, None).await?;

            let (tx, rx) = futures::channel::mpsc::unbounded();
            let message_stream = client.messages();

            tokio::task::spawn_local(async move {
                futures::pin_mut!(message_stream);

                while let Some(message) = message_stream.next().await {
                    match message {
                        Message::Assistant { content } => {
                            if tx.unbounded_send(content).is_err() {
                                break;
                            }
                        }
                        Message::TaskFinish { .. } => {
                            break;
                        }
                        _ => {}
                    }
                }

                let _ = client.disconnect().await;
            });

            Ok(rx)
        })
        .await
}