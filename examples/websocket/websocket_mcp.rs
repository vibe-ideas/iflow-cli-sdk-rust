//! WebSocket client example showing MCP server configuration with iFlow Rust SDK
//!
//! This example demonstrates how to configure MCP servers for extended capabilities
//! such as filesystem access when using WebSocket connection.

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{EnvVariable, IFlowClient, IFlowOptions, McpServer, Message};
use iflow_cli_sdk_rust::error::IFlowError;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 Starting iFlow WebSocket client with MCP example...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    let timeout_secs = 300.0;
    local
        .run_until(async {
            // Configure MCP servers for extended capabilities
            let mcp_servers = vec![
                McpServer::Stdio {
                    name: "sequential-thinking".to_string(),
                    command: PathBuf::from("npx"),
                    args: vec!["-y".to_string(), "@iflow-mcp/server-sequential-thinking@0.6.2".to_string()],
                    env: vec![
                        EnvVariable {
                            name: "DEBUG".to_string(),
                            value: "1".to_string(),
                            meta: None,
                        }
                    ],
                }
            ];

            // Configure client options with WebSocket configuration and MCP servers
            let options = IFlowOptions::new()
                .with_websocket_config(iflow_cli_sdk_rust::config::WebSocketConfig::auto_start())
                .with_mcp_servers(mcp_servers)
                // Set a reasonable timeout
                .with_timeout(timeout_secs)
                .with_logging_config(iflow_cli_sdk_rust::config::LoggingConfig {
                    enabled: true,
                    level: "INFO".to_string(),
                    logger_config: iflow_cli_sdk_rust::logger::LoggerConfig {
                        enabled: true,
                        log_file: "logs/iflow_client_websocket_mcp.log".into(),
                        max_file_size: 10 * 1024 * 1024, // 10MB
                        max_files: 5,
                    },
                })
                .with_permission_mode(iflow_cli_sdk_rust::message::types::PermissionMode::Auto);

            // Create and connect client
            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to iFlow via WebSocket with MCP servers...");
            client.connect().await?;
            println!("✅ Connected to iFlow via WebSocket with MCP servers");

            // Receive and process responses
            println!("📥 Receiving responses...");
            let mut message_stream = client.messages();

            let message_task = tokio::task::spawn_local(async move {
                let mut stdout = std::io::stdout();

                while let Some(message) = message_stream.next().await {
                    match message {
                        Message::Assistant { content } => {
                            print!("{}", content);
                            stdout
                                .flush()
                                .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;
                        }
                        Message::ToolCall { id, name, status } => {
                            println!("\n🔧 Tool call: {} ({}) {}", id, name, status);
                        }
                        Message::Plan { entries } => {
                            println!("\n📋 Plan update received: {:?}", entries);
                        }
                        Message::TaskFinish { .. } => {
                            println!("\n✅ Task completed");
                            break;
                        }
                        Message::Error {
                            code,
                            message: msg,
                            details: _,
                        } => {
                            eprintln!("\n❌ Error {}: {}", code, msg);
                            break;
                        }
                        Message::User { content } => {
                            println!("\n👤 User message: {}", content);
                        }
                    }
                }

                Ok::<(), Box<dyn std::error::Error>>(())
            });

            // Send a message that uses MCP capabilities
            let prompt = "List files in the current directory and calculate total file count";
            println!("📤 Sending: {}", prompt);
            
            // Handle the send_message result to catch timeout errors
            match client.send_message(prompt, None).await {
                Ok(()) => {
                    println!("✅ Message sent successfully");
                }
                Err(IFlowError::Timeout(msg)) => {
                    eprintln!("⏰ Timeout error occurred: {}", msg);
                    eprintln!("This may be due to MCP server startup time or processing delays.");
                    eprintln!("Consider increasing the timeout or checking MCP server configuration.");
                }
                Err(e) => {
                    eprintln!("❌ Error sending message: {}", e);
                    return Err(e.into());
                }
            }

            // Wait for the message handling task to finish
            match tokio::time::timeout(
                std::time::Duration::from_secs(timeout_secs as u64),  // Match client timeout
                message_task,
            )
            .await
            {
                Ok(Ok(Ok(()))) => {
                    println!("✅ Message handling completed successfully");
                }
                Ok(Ok(Err(err))) => {
                    eprintln!("❌ Error in message handling: {}", err);
                }
                Ok(Err(err)) => {
                    eprintln!("❌ Message task panicked: {}", err);
                }
                Err(_) => {
                    println!("⏰ Timeout waiting for message handling to complete");
                }
            }

            // Disconnect
            println!("\n🔌 Disconnecting...");
            client.disconnect().await?;
            println!("👋 Disconnected from iFlow");

            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .await
}