//! Test message stream real-time performance

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Test message stream real-time performance");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let options = IFlowOptions::new().with_auto_start_process(true);

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to iFlow...");
            client.connect().await?;
            println!("✅ Connection successful");

            // Send a simple message
            let prompt = "Please say 'Hello World' and then finish";
            println!("📤 Sending message: {}", prompt);
            client.send_message(prompt, None).await?;

            println!("📥 Start receiving messages...");
            let start_time = Instant::now();
            let mut message_count = 0;

            let mut message_stream = client.messages();
            let mut finished = false;

            while !finished {
                // Use timeout for each message
                match tokio::time::timeout(std::time::Duration::from_secs(30), message_stream.next()).await {
                    Ok(Some(message)) => {
                        message_count += 1;
                        let elapsed = start_time.elapsed();

                        match message {
                            Message::Assistant { content } => {
                                println!("[{:.2}s] 💬 Assistant: {}", elapsed.as_secs_f32(), content);
                            }
                            Message::ToolCall { id, name, status } => {
                                println!(
                                    "[{:.2}s] 🔧 ToolCall: {} ({}): {}",
                                    elapsed.as_secs_f32(),
                                    id,
                                    name,
                                    status
                                );
                            }
                            Message::Plan { entries } => {
                                println!("[{:.2}s] 📋 Plan: {:?}", elapsed.as_secs_f32(), entries);
                            }
                            Message::TaskFinish { reason } => {
                                println!(
                                    "[{:.2}s] ✅ TaskFinish: {:?}",
                                    elapsed.as_secs_f32(),
                                    reason
                                );
                                finished = true;
                            }
                            Message::Error { code, message: msg, details: _ } => {
                                println!("[{:.2}s] ❌ Error {}: {}", elapsed.as_secs_f32(), code, msg);
                                finished = true;
                            }
                            Message::User { content } => {
                                println!("[{:.2}s] 👤 User: {}", elapsed.as_secs_f32(), content);
                            }
                        }

                        // Flush stdout to ensure real-time display
                        std::io::stdout().flush()?;
                    }
                    Ok(None) => {
                        // Stream ended
                        finished = true;
                    }
                    Err(_) => {
                        // Timeout
                        println!("⏰ Timeout waiting for next message");
                        finished = true;
                    }
                }
            }

            println!(
                "📊 Test completed: Received {} messages in {:.2} seconds",
                message_count,
                start_time.elapsed().as_secs_f32()
            );

            client.disconnect().await?;
            println!("👋 Disconnected");

            Ok(())
        })
        .await
}