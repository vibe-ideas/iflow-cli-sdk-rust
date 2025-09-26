//! Basic client example showing bidirectional communication with iFlow

use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    println!("🚀 Starting iFlow client example...");
    
    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Configure client options
        let options = IFlowOptions::new()
            .with_auto_start_process(true);
        
        // Create and connect client
        let mut client = IFlowClient::new(Some(options));
        
        println!("🔗 Connecting to iFlow...");
        client.connect().await?;
        println!("✅ Connected to iFlow");
        
        // Send a message
        let prompt = "Hello iFlow! Can you help me write a simple Rust function that calculates the factorial of a number?";
        println!("📤 Sending: {}", prompt);
        client.send_message(prompt, None).await?;
        
        // Receive and process responses
        println!("📥 Receiving responses...");
        let mut message_stream = client.messages();
        
        while let Some(message) = message_stream.next().await {
            // Handle different message types
            match message {
                Message::Assistant { content } => {
                    print!("{}", content);
                    use std::io::{self, Write};
                    io::stdout().flush()?;
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
                }
                Message::User { content } => {
                    println!("\n👤 User message: {}", content);
                }
            }
        }
        
        // Disconnect
        println!("\n🔌 Disconnecting...");
        client.disconnect().await?;
        println!("👋 Disconnected from iFlow");
        
        Ok(())
    }).await
}