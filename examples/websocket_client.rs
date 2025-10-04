//! WebSocket client example showing bidirectional communication with iFlow

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting iFlow WebSocket client example...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Configure client options with WebSocket configuration and custom timeout
        let custom_timeout_secs = 120.0;
        let options = IFlowOptions::new()
            .with_websocket_config(iflow_cli_sdk_rust::types::WebSocketConfig::auto_start())
            .with_timeout(custom_timeout_secs)
            .with_process_config(iflow_cli_sdk_rust::types::ProcessConfig::new().enable_auto_start().start_port(8090));

        // Create and connect client
        let mut client = IFlowClient::new(Some(options));

        println!("🔗 Connecting to iFlow via WebSocket...");
        client.connect().await?;
        println!("✅ Connected to iFlow via WebSocket");

        // Receive and process responses
        println!("📥 Receiving responses...");
        let mut message_stream = client.messages();

        let message_task = tokio::task::spawn_local(async move {
            let mut stdout = std::io::stdout();

            while let Some(message) = message_stream.next().await {
                match message {
                    Message::Assistant { content } => {
                        print!("{}", content);
                        stdout.flush().map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;
                    }
                    Message::ToolCall { id, name, status } => {
                        println!("\n🔧 Tool call: {} ({}): {}", id, name, status);
                    }
                    Message::Plan { entries } => {
                        println!("\n📋 Plan update received: {:?}", entries);
                    }
                    Message::TaskFinish { .. } => {
                        println!("\n✅ Task completed");
                        break;
                    }
                    Message::Error { code, message: msg, details: _ } => {
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

        // Send a message
        let prompt = "Create a plan to introduce this project.";
        println!("📤 Sending: {}", prompt);
        client.send_message(prompt, None).await?;

        // Wait for the message handling task to finish with a timeout
        match tokio::time::timeout(std::time::Duration::from_secs_f64(custom_timeout_secs), message_task).await {
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
    }).await
}