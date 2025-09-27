//! 日志功能示例 - 记录原始 iflow 消息（Debug 格式）

use iflow_cli_sdk_rust::{LoggerConfig, Message, MessageLogger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("iFlow 原始消息日志记录示例（Debug 格式）");

    // 创建自定义配置，启用日志记录
    let config = LoggerConfig {
        log_file: std::path::PathBuf::from("logs/iflow_messages.log"),
        enabled: true,
        max_file_size: 1024 * 1024, // 1MB
        max_files: 5,
    };

    let logger = MessageLogger::new(config)?;
    println!("日志记录器初始化成功");

    // 记录各种类型的原始消息
    println!("记录原始测试消息...");

    // 用户消息
    let user_msg = Message::User {
        content: "Hello, iFlow! This is a test message for logging.".to_string(),
    };
    logger.log_message(&user_msg).await?;

    // 助手消息
    let assistant_msg = Message::Assistant {
        content: "Hello! I'm the assistant. This message is being logged.".to_string(),
    };
    logger.log_message(&assistant_msg).await?;

    // 工具调用消息
    let tool_call_msg = Message::ToolCall {
        id: "12345".to_string(),
        name: "read_file".to_string(),
        status: "completed".to_string(),
    };
    logger.log_message(&tool_call_msg).await?;

    // 计划消息
    let plan_msg = Message::Plan {
        entries: vec![
            "第一步：分析需求".to_string(),
            "第二步：实现功能".to_string(),
            "第三步：测试验证".to_string(),
        ],
    };
    logger.log_message(&plan_msg).await?;

    // 任务完成消息
    let finish_msg = Message::TaskFinish {
        reason: Some("completed successfully".to_string()),
    };
    logger.log_message(&finish_msg).await?;

    println!("所有原始消息已记录到日志文件");
    println!("日志文件位置：logs/iflow_messages.log");

    // 显示日志文件内容（Debug 格式）
    println!("\n日志文件内容（Debug 格式）：");
    if let Ok(content) = std::fs::read_to_string("logs/iflow_messages.log") {
        for line in content.lines() {
            println!("{}", line);
        }
    } else {
        println!("无法读取日志文件");
    }

    println!("\n原始消息日志记录功能测试完成！");

    Ok(())
}
