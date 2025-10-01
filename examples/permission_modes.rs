//! Example showing different permission modes with iFlow

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use iflow_cli_sdk_rust::types::PermissionMode;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting iFlow permission modes example...");

    // Demonstrate different permission modes
    demonstrate_permission_mode(PermissionMode::Auto, "Auto").await?;
    demonstrate_permission_mode(PermissionMode::Selective, "Selective").await?;
    demonstrate_permission_mode(PermissionMode::Manual, "Manual").await?;

    Ok(())
}

async fn demonstrate_permission_mode(mode: PermissionMode, mode_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== {} Permission Mode ===", mode_name);

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Configure client options with WebSocket URL and specific permission mode
        let options = IFlowOptions::new()
            .with_websocket_url("ws://localhost:8090/acp?peer=iflow")
            .with_auto_start_process(true)
            .with_permission_mode(mode);

        // Create and connect client
        let mut client = IFlowClient::new(Some(options));

        println!("🔗 Connecting to iFlow via WebSocket with {} permission mode...", mode_name);
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
                    Message::Error { code, message: msg } => {
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

        // Send a message that might trigger tool calls
        let prompt = "列出当前目录的文件，并创建一个名为 test.txt 的文件，内容为 'Hello, iFlow!'";
        println!("📤 Sending: {}", prompt);
        client.send_message(prompt, None).await?;

        // Wait for the message handling task to finish with a timeout
        match tokio::time::timeout(std::time::Duration::from_secs(60), message_task).await {
            Ok(Ok(Ok(()))) => {
                println!("✅ Message handling completed successfully");
            }
            Ok(Ok(Err(err))) => {
                eprintln!("❌ Error in message handling: {}", err);
                return Err(err);
            }
            Ok(Err(err)) => {
                eprintln!("❌ Message task panicked: {}", err);
                return Err(Box::new(err));
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
    }).await?;

    Ok(())
}
