//! Simple query example using the convenience function

use iflow_cli_sdk_rust::query_with_timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting simple query example...");

    // Simple query with custom timeout
    let prompt = "What is the capital of France? Please provide a brief answer.";
    println!("❓ Query: {}", prompt);

    match query_with_timeout(prompt, 120.0).await {
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
