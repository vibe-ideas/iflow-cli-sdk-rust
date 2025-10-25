use crate::client::IFlowClient;
use crate::error::Result;
use crate::config::options::IFlowOptions;
use crate::message::types::Message;
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
    let default_timeout = IFlowOptions::default().timeout;
    query_with_timeout(prompt, default_timeout).await
}

/// Simple synchronous query to iFlow with custom options
///
/// Sends a query to iFlow and waits for a complete response.
/// This is a convenience function for simple request-response interactions.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
/// * `options` - Configuration options for the query
///
/// # Returns
/// * `Ok(String)` containing the response from iFlow
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::{query_with_config, IFlowOptions};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let options = IFlowOptions::new().with_timeout(120.0);
///     let response = query_with_config("What is 2 + 2?", options).await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
pub async fn query_with_config(prompt: &str, options: IFlowOptions) -> Result<String> {
    // Apply timeout to the entire operation
    let timeout_secs = options.timeout;
    // Use a fraction of the total timeout for individual message reception
    // This ensures we don't block indefinitely on any single message
    let message_timeout_secs = (timeout_secs / 10.0).clamp(0.1, 1.0);

    match timeout(Duration::from_secs_f64(timeout_secs), async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                tracing::debug!("Creating IFlowClient with custom options");
                let mut client = IFlowClient::new(Some(options));
                tracing::debug!("Connecting to iFlow...");
                client.connect().await?;
                tracing::debug!("Connected to iFlow");

                tracing::debug!("Sending message: {}", prompt);
                client.send_message(prompt, None).await?;
                tracing::debug!("Message sent");

                let mut response = String::new();
                let mut message_stream = client.messages();

                // First wait for the send_message to complete by receiving the TaskFinish message
                // The send_message function sends a TaskFinish message when the prompt is complete
                let mut prompt_finished = false;
                while !prompt_finished {
                    match timeout(
                        Duration::from_secs_f64(message_timeout_secs),
                        message_stream.next(),
                    )
                    .await
                    {
                        Ok(Some(message)) => {
                            tracing::debug!("Received message: {:?}", message);
                            match message {
                                Message::Assistant { content } => {
                                    response.push_str(&content);
                                }
                                Message::TaskFinish { .. } => {
                                    prompt_finished = true;
                                }
                                _ => {}
                            }
                        }
                        Ok(None) => {
                            // Stream ended
                            tracing::debug!("Message stream ended");
                            prompt_finished = true;
                        }
                        Err(_) => {
                            // Timeout on individual message - this is expected during normal operation
                            // Continue the loop to check if we should still wait
                            // The outer timeout will catch if we've exceeded the total time
                        }
                    }
                }
                tracing::debug!("Query completed, response length: {}", response.len());

                client.disconnect().await?;
                Ok(response.trim().to_string())
            })
            .await
    })
    .await
    {
        Ok(result) => result,
        Err(_) => Err(crate::error::IFlowError::Timeout(
            "Operation timed out".to_string(),
        )),
    }
}

/// Simple synchronous query to iFlow with custom timeout
///
/// Sends a query to iFlow and waits for a complete response.
/// This is a convenience function for simple request-response interactions.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
/// * `timeout_secs` - Timeout in seconds
///
/// # Returns
/// * `Ok(String)` containing the response from iFlow
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::query_with_timeout;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let response = query_with_timeout("What is 2 + 2?", 120.0).await?;
///     println!("{}", response);
///     Ok(())
/// }
/// ```
pub async fn query_with_timeout(prompt: &str, timeout_secs: f64) -> Result<String> {
    // Apply timeout to the entire operation
    // Use a fraction of the total timeout for individual message reception
    // This ensures we don't block indefinitely on any single message
    let message_timeout_secs = (timeout_secs / 10.0).clamp(0.1, 1.0);

    match timeout(Duration::from_secs_f64(timeout_secs), async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                // Create client with the specified timeout and auto-start configuration for stdio mode
                let options = IFlowOptions::new()
                    .with_timeout(timeout_secs)
                    .with_process_config(
                        crate::config::process::ProcessConfig::new()
                            .enable_auto_start()
                            .stdio_mode(),
                    );
                tracing::debug!(
                    "Creating IFlowClient with options: auto_start={}, start_port={:?}",
                    options.process.auto_start,
                    options.process.start_port
                );
                let mut client = IFlowClient::new(Some(options));
                tracing::debug!("Connecting to iFlow...");
                client.connect().await?;
                tracing::debug!("Connected to iFlow");

                tracing::debug!("Sending message: {}", prompt);
                client.send_message(prompt, None).await?;
                tracing::debug!("Message sent");

                let mut response = String::new();
                let mut message_stream = client.messages();

                // First wait for the send_message to complete by receiving the TaskFinish message
                // The send_message function sends a TaskFinish message when the prompt is complete
                let mut prompt_finished = false;
                while !prompt_finished {
                    match timeout(
                        Duration::from_secs_f64(message_timeout_secs),
                        message_stream.next(),
                    )
                    .await
                    {
                        Ok(Some(message)) => {
                            tracing::debug!("Received message: {:?}", message);
                            match message {
                                Message::Assistant { content } => {
                                    response.push_str(&content);
                                }
                                Message::TaskFinish { .. } => {
                                    prompt_finished = true;
                                }
                                _ => {}
                            }
                        }
                        Ok(None) => {
                            // Stream ended
                            tracing::debug!("Message stream ended");
                            prompt_finished = true;
                        }
                        Err(_) => {
                            // Timeout on individual message - this is expected during normal operation
                            // Continue the loop to check if we should still wait
                            // The outer timeout will catch if we've exceeded the total time
                        }
                    }
                }
                tracing::debug!("Query completed, response length: {}", response.len());

                client.disconnect().await?;
                Ok(response.trim().to_string())
            })
            .await
    })
    .await
    {
        Ok(result) => result,
        Err(_) => Err(crate::error::IFlowError::Timeout(
            "Operation timed out".to_string(),
        )),
    }
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
    query_stream_with_timeout(prompt, 120.0).await
}

/// Stream responses from iFlow with custom options
///
/// Sends a query to iFlow and returns a stream of response chunks.
/// This is useful for real-time output as the response is generated.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
/// * `options` - Configuration options for the query
///
/// # Returns
/// * `Ok(impl Stream<Item = String>)` containing the response stream
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::{query_stream_with_config, IFlowOptions};
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let options = IFlowOptions::new().with_timeout(60.0);
///     let mut stream = query_stream_with_config("Tell me a story", options).await?;
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
pub async fn query_stream_with_config(
    prompt: &str,
    options: IFlowOptions,
) -> Result<impl futures::Stream<Item = String>> {
    let local = tokio::task::LocalSet::new();
    // We need to run this in a LocalSet context but return a stream
    // Let's create the client and connection in the LocalSet context
    local
        .run_until(async {
            // Create client with the specified options
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

/// Stream responses from iFlow with custom timeout
///
/// Sends a query to iFlow and returns a stream of response chunks.
/// This is useful for real-time output as the response is generated.
///
/// # Arguments
/// * `prompt` - The query prompt to send to iFlow
/// * `timeout_secs` - Timeout in seconds
///
/// # Returns
/// * `Ok(impl Stream<Item = String>)` containing the response stream
/// * `Err(IFlowError)` if there was an error
///
/// # Example
/// ```no_run
/// use iflow_cli_sdk_rust::query_stream_with_timeout;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut stream = query_stream_with_timeout("Tell me a story", 120.0).await?;
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
pub async fn query_stream_with_timeout(
    prompt: &str,
    timeout_secs: f64,
) -> Result<impl futures::Stream<Item = String>> {
    let local = tokio::task::LocalSet::new();
    // We need to run this in a LocalSet context but return a stream
    // Let's create the client and connection in the LocalSet context
    local
        .run_until(async {
            // Create client with the specified timeout
            let options = IFlowOptions::new().with_timeout(timeout_secs);
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
