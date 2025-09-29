//! Test WebSocket functionality

use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions};

#[tokio::test]
async fn test_websocket_connection() {
    // This test requires iFlow to be running on localhost:8090
    // It's meant to be run manually, not in CI
    
    /*
    let options = IFlowOptions::new()
        .with_websocket_url("ws://localhost:8090/acp")
        .with_auto_start_process(true);
    
    let mut client = IFlowClient::new(Some(options));
    
    // Try to connect
    match client.connect().await {
        Ok(()) => {
            println!("Connected successfully");
            
            // Try to disconnect
            match client.disconnect().await {
                Ok(()) => println!("Disconnected successfully"),
                Err(e) => println!("Error disconnecting: {}", e),
            }
        }
        Err(e) => {
            println!("Connection failed: {}", e);
        }
    }
    */
    
    println!("WebSocket test - manual execution required");
}