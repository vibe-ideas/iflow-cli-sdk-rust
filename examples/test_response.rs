//! Test to verify iFlow response handling

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use iflow_cli_sdk_rust::error::IFlowError;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🧪 Testing iFlow response handling...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            // Configure client options with auto-start enabled for stdio mode
            let options = IFlowOptions::new().with_timeout(30.0).with_process_config(
                iflow_cli_sdk_rust::types::ProcessConfig::new()
                    .enable_auto_start()
                    .stdio_mode(),
            );

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to iFlow...");
            client.connect().await?;
            println!("✅ Connected to iFlow");

            // Send a simple test message
            let prompt = "Hello! Please reply with 'Hello back!' to confirm you're working.";
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

            // Wait for response with timeout
            println!("⏳ Waiting for response...");
            let mut message_stream = client.messages();

            let result = timeout(Duration::from_secs(10), async {
                let mut response_received = false;

                while let Some(message) = message_stream.next().await {
                    match message {
                        Message::Assistant { content } => {
                            println!("📝 Received assistant response: {}", content);
                            if content.contains("Hello back") || content.contains("hello") {
                                println!("✅ SUCCESS: Received expected response!");
                                response_received = true;
                                break;
                            }
                        }
                        Message::User { content } => {
                            println!("👤 User message echo: {}", content);
                            if content.contains("Hello!") {
                                println!("ℹ️  Received our own message echo");
                            }
                        }
                        Message::TaskFinish { reason } => {
                            println!("🏁 Task finished: {:?}", reason);
                            break;
                        }
                        Message::Error {
                            code,
                            message,
                            details: _,
                        } => {
                            println!("❌ Error {}: {}", code, message);
                            break;
                        }
                        _ => {
                            println!("📨 Other message type: {:?}", message);
                        }
                    }
                }

                response_received
            })
            .await;

            match result {
                Ok(true) => {
                    println!("🎉 TEST PASSED: Received expected response from iFlow!");
                }
                Ok(false) => {
                    println!("⚠️  TEST INCONCLUSIVE: No matching response received");
                }
                Err(_) => {
                    println!("⏰ TEST FAILED: Timeout waiting for response");
                }
            }

            // Disconnect
            println!("\n🔌 Disconnecting...");
            client.disconnect().await?;
            println!("👋 Disconnected from iFlow");

            Ok(())
        })
        .await
}