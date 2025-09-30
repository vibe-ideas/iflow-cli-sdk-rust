//! Simple query example using the convenience function

use iflow_cli_sdk_rust::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    println!("🚀 Starting simple query example...");

    // Simple query
    let prompt = "What is the capital of France? Please provide a brief answer.";
    println!("❓ Query: {}", prompt);

    match query(prompt).await {
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