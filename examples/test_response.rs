//! Test to verify iFlow response handling

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🧪 Testing iFlow response handling...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            // Configure client options
            let options = IFlowOptions::new()
                .with_timeout(30.0)
                .with_auto_start_process(true);

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to iFlow...");
            client.connect().await?;
            println!("✅ Connected to iFlow");

            // Send a simple test message
            let prompt = "Hello! Please reply with 'Hello back!' to confirm you're working.";
            println!("📤 Sending: {}", prompt);
            client.send_message(prompt, None).await?;

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
                        Message::Error { code, message } => {
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
