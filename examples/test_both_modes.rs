//! Test both stdio and WebSocket modes to verify they work correctly

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Testing both stdio and WebSocket modes...");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Test 1: stdio mode
        println!("\n=== Testing stdio mode ===");
        match test_stdio_mode().await {
            Ok(()) => println!("✅ stdio mode test completed"),
            Err(e) => println!("❌ stdio mode test failed: {}", e),
        }

        // Test 2: WebSocket mode (only if we can start iFlow)
        println!("\n=== Testing WebSocket mode ===");
        match test_websocket_mode().await {
            Ok(()) => println!("✅ WebSocket mode test completed"),
            Err(e) => println!("❌ WebSocket mode test failed: {}", e),
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    }).await
}

async fn test_stdio_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Configure client options for stdio
    let options = IFlowOptions::new()
        .with_auto_start_process(true);

    // Create and connect client
    let mut client = IFlowClient::new(Some(options));

    println!("🔗 Connecting to iFlow via stdio...");
    client.connect().await?;
    println!("✅ Connected to iFlow via stdio");

    // Test sending a simple message
    let prompt = "Hello from stdio mode";
    println!("📤 Sending: {}", prompt);
    
    // Set up message receiving with timeout
    let mut message_stream = client.messages();
    let message_task = tokio::task::spawn_local(async move {
        let mut received_messages = 0;
        let mut stdout = std::io::stdout();

        while let Some(message) = message_stream.next().await {
            received_messages += 1;
            match message {
                Message::Assistant { content } => {
                    print!("📥 Assistant: {}", content);
                    stdout.flush()?;
                }
                Message::TaskFinish { .. } => {
                    println!("\n✅ Task completed");
                    break;
                }
                Message::Error { code, message: msg } => {
                    println!("\n❌ Error {}: {}", code, msg);
                    break;
                }
                _ => {
                    println!("\n📨 Other message: {:?}", message);
                }
            }
            
            // Limit message processing to avoid infinite loops
            if received_messages > 10 {
                println!("\n⚠️  Received {} messages, stopping", received_messages);
                break;
            }
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    });

    // Send the message
    let send_result = client.send_message(prompt, None).await;
    
    // Wait for either the message task to complete or timeout
    let timeout_duration = Duration::from_secs(10);
    match tokio::time::timeout(timeout_duration, message_task).await {
        Ok(Ok(Ok(()))) => {
            println!("✅ Message handling completed successfully");
        }
        Ok(Ok(Err(err))) => {
            println!("❌ Error in message handling: {}", err);
        }
        Ok(Err(err)) => {
            println!("❌ Message task panicked: {}", err);
        }
        Err(_) => {
            println!("⏰ Timeout waiting for message handling to complete");
        }
    }

    // Check send result
    match send_result {
        Ok(()) => println!("✅ Message sent successfully"),
        Err(e) => println!("❌ Failed to send message: {}", e),
    }

    // Disconnect
    println!("🔌 Disconnecting from stdio...");
    client.disconnect().await?;
    println!("👋 Disconnected from stdio");

    Ok(())
}

async fn test_websocket_mode() -> Result<(), Box<dyn std::error::Error>> {
    // Configure client options for WebSocket
    let options = IFlowOptions::new()
        .with_websocket_url("ws://localhost:8090/acp")
        .with_auto_start_process(true);

    // Create and connect client
    let mut client = IFlowClient::new(Some(options));

    println!("🔗 Connecting to iFlow via WebSocket...");
    match client.connect().await {
        Ok(()) => println!("✅ Connected to iFlow via WebSocket"),
        Err(e) => {
            println!("❌ Failed to connect via WebSocket: {}", e);
            return Ok(()); // Don't fail the test if WebSocket is not available
        }
    }

    // Test sending a simple message
    let prompt = "Hello from WebSocket mode";
    println!("📤 Sending: {}", prompt);
    
    // Set up message receiving with timeout
    let mut message_stream = client.messages();
    let message_task = tokio::task::spawn_local(async move {
        let mut received_messages = 0;
        let mut stdout = std::io::stdout();

        while let Some(message) = message_stream.next().await {
            received_messages += 1;
            match message {
                Message::Assistant { content } => {
                    print!("📥 Assistant: {}", content);
                    stdout.flush()?;
                }
                Message::TaskFinish { .. } => {
                    println!("\n✅ Task completed");
                    break;
                }
                Message::Error { code, message: msg } => {
                    println!("\n❌ Error {}: {}", code, msg);
                    break;
                }
                _ => {
                    println!("\n📨 Other message: {:?}", message);
                }
            }
            
            // Limit message processing to avoid infinite loops
            if received_messages > 10 {
                println!("\n⚠️  Received {} messages, stopping", received_messages);
                break;
            }
        }

        Ok::<(), Box<dyn std::error::Error>>(())
    });

    // Send the message
    let send_result = client.send_message(prompt, None).await;
    
    // Wait for either the message task to complete or timeout
    let timeout_duration = Duration::from_secs(10);
    match tokio::time::timeout(timeout_duration, message_task).await {
        Ok(Ok(Ok(()))) => {
            println!("✅ Message handling completed successfully");
        }
        Ok(Ok(Err(err))) => {
            println!("❌ Error in message handling: {}", err);
        }
        Ok(Err(err)) => {
            println!("❌ Message task panicked: {}", err);
        }
        Err(_) => {
            println!("⏰ Timeout waiting for message handling to complete");
        }
    }

    // Check send result
    match send_result {
        Ok(()) => println!("✅ Message sent successfully"),
        Err(e) => println!("❌ Failed to send message: {}", e),
    }

    // Disconnect
    println!("🔌 Disconnecting from WebSocket...");
    client.disconnect().await?;
    println!("👋 Disconnected from WebSocket");

    Ok(())
}