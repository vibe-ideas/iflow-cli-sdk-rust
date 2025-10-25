//! Example demonstrating MCP server configuration with iFlow Rust SDK
//!
//! This example shows how to configure MCP servers for extended capabilities
//! such as filesystem access, command execution, etc.

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{EnvVariable, IFlowClient, IFlowOptions, McpServer};
use iflow_cli_sdk_rust::error::IFlowError;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Use LocalSet for proper async runtime compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        use std::path::PathBuf;

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

        // Create options with MCP server configuration
        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_process_config(
                    iflow_cli_sdk_rust::config::ProcessConfig::new()
                        .enable_auto_start()
                        .start_port(8090)
            )
            .with_logging_config(iflow_cli_sdk_rust::config::LoggingConfig {
                    enabled: true,
                    level: "INFO".to_string(),
                    logger_config: iflow_cli_sdk_rust::logger::LoggerConfig {
                        enabled: true,
                        log_file: "logs/iflow_client_mcp.log".into(),
                        max_file_size: 10 * 1024 * 1024, // 10MB
                        max_files: 5,
                    },
                });

        // Create client with options
        let mut client = IFlowClient::new(Some(options));

        // Connect to iFlow
        client.connect().await?;

        // Send a message that use MCP capabilities
        let prompt = "use sequential-thinking mcp server List files in the current directory, calc total font nums";
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

        // Listen for messages
        let mut message_stream = client.messages();
        while let Some(message) = message_stream.next().await {
            match message {
                iflow_cli_sdk_rust::Message::Assistant { content } => {
                    print!("{}", content);
                    std::io::stdout().flush()?;
                }
                iflow_cli_sdk_rust::Message::TaskFinish { .. } => {
                    break;
                }
                _ => {
                    // Handle other message types
                    println!("Received other message type");
                }
            }
        }

        // Disconnect from iFlow
        client.disconnect().await?;

        Ok(())
    }).await
}