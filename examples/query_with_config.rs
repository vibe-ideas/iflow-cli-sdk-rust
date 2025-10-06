//! Simple query example using the convenience function with custom configuration

use iflow_cli_sdk_rust::{IFlowOptions, query_with_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting query with config example...");

    // Query with custom timeout
    let prompt = "What is the capital of France? Please provide a brief answer.";
    println!("❓ Query: {}", prompt);

    // Create custom options with a 60-second timeout
    let options = IFlowOptions::new().with_timeout(60.0);

    match query_with_config(prompt, options).await {
        Ok(response) => {
            println!("💡 Answer: {}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }

    println!("✅ Query completed successfully");
    Ok(())
}
