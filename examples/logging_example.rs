//! Logging example - Record raw iflow messages (Debug format)

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};
use iflow_cli_sdk_rust::types::{PlanEntry, PlanPriority, PlanStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("iFlow raw message logging example (Debug format)");

    // Create custom configuration with logging enabled
    let config = LoggerConfig {
        log_file: std::path::PathBuf::from("logs/iflow_messages.log"),
        enabled: true,
        max_file_size: 1024 * 1024, // 1MB
        max_files: 5,
    };

    let logger = MessageLogger::new(config)?;
    println!("Logger initialized successfully");

    // Log various types of raw messages
    println!("Logging raw test messages...");

    // User message
    let user_msg = Message::User {
        content: "Hello, iFlow! This is a test message for logging.".to_string(),
    };
    logger.log_message(&user_msg).await?;

    // Assistant message
    let assistant_msg = Message::Assistant {
        content: "Hello! I'm the assistant. This message is being logged.".to_string(),
    };
    logger.log_message(&assistant_msg).await?;

    // Tool call message
    let tool_call_msg = Message::ToolCall {
        id: "12345".to_string(),
        name: "read_file".to_string(),
        status: "completed".to_string(),
    };
    logger.log_message(&tool_call_msg).await?;

    // Plan message
    let plan_msg = Message::Plan {
        entries: vec![
            PlanEntry {
                content: "Step 1: Analyze requirements".to_string(),
                priority: PlanPriority::Medium,
                status: PlanStatus::Pending,
            },
            PlanEntry {
                content: "Step 2: Implement features".to_string(),
                priority: PlanPriority::High,
                status: PlanStatus::InProgress,
            },
            PlanEntry {
                content: "Step 3: Test and verify".to_string(),
                priority: PlanPriority::Medium,
                status: PlanStatus::Pending,
            },
        ],
    };
    logger.log_message(&plan_msg).await?;

    // Task finish message
    let finish_msg = Message::TaskFinish {
        reason: Some("completed successfully".to_string()),
    };
    logger.log_message(&finish_msg).await?;

    println!("All raw messages have been logged to the log file");
    println!("Log file location: logs/iflow_messages.log");

    // Display log file content (Debug format)
    println!("\nLog file content (Debug format):\n");
    if let Ok(content) = std::fs::read_to_string("logs/iflow_messages.log") {
        for line in content.lines() {
            println!("{}", line);
        }
    } else {
        println!("Failed to read log file");
    }

    println!("\nRaw message logging feature test completed!");

    Ok(())
}
