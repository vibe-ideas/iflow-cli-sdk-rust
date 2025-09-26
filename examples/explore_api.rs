use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing the new iFlow client implementation...");
    
    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local.run_until(async {
        // Create a client with options
        let options = IFlowOptions::new();
        
        let mut client = IFlowClient::new(Some(options));
        
        // Try to connect (this will fail if iFlow is not running, but we can see the connection attempt)
        match client.connect().await {
            Ok(_) => {
                println!("✅ Successfully connected to iFlow!");
                
                // Send a test message
                client.send_message("Hello iFlow!", None).await?;
                println!("✅ Message sent successfully!");
                
                // Disconnect
                client.disconnect().await?;
                println!("✅ Disconnected successfully!");
            }
            Err(e) => {
                println!("❌ Connection failed: {}", e);
                println!("This is expected if iFlow is not installed or not in PATH");
            }
        }
        
        Ok(())
    }).await
}