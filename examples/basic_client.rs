//! Basic client example showing bidirectional communication with iFlow

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting iFlow client example...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Configure client options
        let options = IFlowOptions::new()
            .with_auto_start_process(true)
            .with_log_file("logs/iflow_client.log");

        // Create and connect client
        let mut client = IFlowClient::new(Some(options));

        println!("🔗 Connecting to iFlow...");
        client.connect().await?;
        println!("✅ Connected to iFlow");

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

        // Send a message
        let prompt = "say hello, how are you?";
        println!("📤 Sending: {}", prompt);
        client.send_message(prompt, None).await?;

        // Wait for the message handling task to finish
        match message_task.await {
            Ok(Ok(())) => {}
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(Box::new(err)),
        }

        // Disconnect
        println!("\n🔌 Disconnecting...");
        client.disconnect().await?;
        println!("👋 Disconnected from iFlow");

        Ok::<(), Box<dyn std::error::Error>>(())
    }).await
}
