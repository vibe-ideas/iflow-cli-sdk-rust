//! WebSocket client example showing connection to an existing iFlow process

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use iflow_cli_sdk_rust::error::IFlowError;
use std::io::Write;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 Starting iFlow WebSocket client example for existing process...");

    // Start an iFlow process manually
    println!("🔧 Starting iFlow process manually...");
    let mut iflow_process = Command::new("iflow")
        .arg("--experimental-acp")
        .arg("--port")
        .arg("8093")
        .spawn()
        .expect("Failed to start iFlow process");

    // Give the process a moment to start - poll until the port is listening
    println!("⏳ Waiting for iFlow process to be ready...");
    let mut attempts = 0;
    let max_attempts = 30; // 30 attempts * 1 second = 30 seconds total

    while attempts < max_attempts {
        if std::net::TcpStream::connect_timeout(
            &"127.0.0.1:8093".parse().unwrap(),
            std::time::Duration::from_millis(100),
        )
        .is_ok()
        {
            println!("✅ iFlow WebSocket server is ready on port 8093");
            break;
        }

        attempts += 1;
        if attempts % 5 == 0 {
            println!(
                "⏳ Still waiting for iFlow to be ready... (attempt {}/{})",
                attempts, max_attempts
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    if attempts >= max_attempts {
        eprintln!(
            "❌ iFlow process failed to start WebSocket server on port 8093 after {} seconds",
            max_attempts
        );
        // Clean up the iFlow process
        let _ = iflow_process.kill();
        let _ = iflow_process.wait();
        return Err("iFlow process failed to start".into());
    }

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            // Configure client options to connect to the existing process
            let custom_timeout_secs = 120.0;
            let options = IFlowOptions::new()
                .with_websocket_config(iflow_cli_sdk_rust::config::WebSocketConfig::new(
                    "ws://localhost:8093/acp?peer=iflow".to_string(),
                ))
                .with_timeout(custom_timeout_secs);

            // Create and connect client
            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to existing iFlow process via WebSocket...");
            client.connect().await?;
            println!("✅ Connected to existing iFlow process via WebSocket");

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

            // Send a message
            let prompt = "Create a plan to introduce this project.";
            println!("📤 Sending: {}", prompt);
            
            // Handle the send_message result to catch timeout errors
            match client.send_message(prompt, None).await {
                Ok(()) => {
                    println!("✅ Message sent successfully");
                }
                Err(IFlowError::Timeout(msg)) => {
                    eprintln!("⏰ Timeout error occurred: {}", msg);
                    eprintln!("This may be due to processing delays.");
                    eprintln!("Consider increasing the timeout or checking the iFlow process.");
                }
                Err(e) => {
                    eprintln!("❌ Error sending message: {}", e);
                    return Err(e.into());
                }
            }

            // Wait for the message handling task to finish with a timeout
            match tokio::time::timeout(
                std::time::Duration::from_secs_f64(custom_timeout_secs),
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
        .await?;

    // Clean up the iFlow process
    println!("🧹 Cleaning up iFlow process...");
    let _ = iflow_process.kill();
    let _ = iflow_process.wait();

    Ok(())
}