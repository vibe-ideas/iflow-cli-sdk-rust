//! Test real-time message receiving

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Test real-time message receiving");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let options = IFlowOptions::new().with_auto_start_process(true);

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 Connecting to iFlow...");
            client.connect().await?;
            println!("✅ Connection successful");

            let start_time = Instant::now();

            // Send message
            println!("📤 [{}s] Sending message...", start_time.elapsed().as_secs_f32());
            client
                .send_message("Please say 'Hello World' and then finish", None)
                .await?;

            println!("📥 Start receiving messages...");
            let mut message_stream = client.messages();
            let mut message_count = 0;

            while let Some(message) = message_stream.next().await {
                message_count += 1;
                let elapsed = start_time.elapsed().as_secs_f32();

                match message {
                    Message::Assistant { content } => {
                        println!("[{:.2}s] 💬 Assistant: {}", elapsed, content);
                        std::io::stdout().flush()?;
                    }
                    Message::ToolCall { id, name, status } => {
                        println!(
                            "[{:.2}s] 🔧 ToolCall: {} ({}): {}",
                            elapsed, id, name, status
                        );
                    }
                    Message::Plan { entries } => {
                        println!("[{:.2}s] 📋 Plan: {:?}", elapsed, entries);
                    }
                    Message::TaskFinish { reason } => {
                        println!("[{:.2}s] ✅ TaskFinish: {:?}", elapsed, reason);
                        break;
                    }
                    Message::Error { code, message: msg } => {
                        println!("[{:.2}s] ❌ Error {}: {}", elapsed, code, msg);
                        break;
                    }
                    Message::User { content } => {
                        println!("[{:.2}s] 👤 User: {}", elapsed, content);
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
