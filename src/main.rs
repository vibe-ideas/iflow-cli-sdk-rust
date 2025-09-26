//! iFlow CLI SDK - Example usage

use iflow_cli_sdk_rust::{query, IFlowClient, IFlowOptions, Message};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    println!("🚀 iFlow CLI SDK Rust Example");
    println!("================================");
    
    // Example 1: Simple query
    println!("\n📋 Example 1: Simple Query");
    match query("What is 2 + 2?").await {
        Ok(response) => println!("💡 Answer: {}", response),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    
    // Example 2: Interactive session
    println!("\n📋 Example 2: Interactive Session");
    let options = IFlowOptions::new()
        .with_auto_start_process(true);
    
    let mut client = IFlowClient::new(Some(options));
    
    match client.connect().await {
        Ok(_) => {
            println!("✅ Connected to iFlow");
            
            let prompt = "Write a haiku about programming";
            println!("❓ Sending: {}", prompt);
            
            if let Err(e) = client.send_message(prompt, None).await {
                eprintln!("❌ Failed to send message: {}", e);
            } else {
                let mut message_stream = client.messages();
                
                println!("📥 Receiving response...");
                while let Some(message) = message_stream.next().await {
                    // Handle different message types
                match message {
                    Message::Assistant { content } => {
                        print!("{}", content);
                        use std::io::{self, Write};
                        io::stdout().flush()?;
                    }
                    
                    Message::TaskFinish { .. } => {
                        break;
                    }
                    _ => {}
                }
                }
                println!(); // New line after response
            }
            
            client.disconnect().await?;
            println!("👋 Disconnected");
        }
        Err(e) => {
            eprintln!("❌ Failed to connect: {}", e);
            eprintln!("💡 Make sure iFlow CLI is installed or enable auto_start_process");
        }
    }
    
    println!("\n✅ All examples completed");
    Ok(())
}
