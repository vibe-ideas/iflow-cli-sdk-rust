//! Example showing different permission modes with iFlow

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::types::PermissionMode;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use iflow_cli_sdk_rust::error::IFlowError;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with environment variable support
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    println!("🚀 Starting iFlow permission modes example...");

    // Demonstrate different permission modes
    demonstrate_permission_mode(PermissionMode::Auto, "Auto").await?;
    demonstrate_permission_mode(PermissionMode::Selective, "Selective").await?;
    demonstrate_permission_mode(PermissionMode::Manual, "Manual").await?;

    Ok(())
}

async fn demonstrate_permission_mode(
    mode: PermissionMode,
    mode_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== {} Permission Mode ===", mode_name);

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            // Configure client options with WebSocket configuration and specific permission mode
            // In auto start mode, we let the SDK generate the WebSocket URL
            let options = IFlowOptions::new()
                .with_websocket_config(iflow_cli_sdk_rust::types::WebSocketConfig::auto_start())
                .with_process_config(
                    iflow_cli_sdk_rust::types::ProcessConfig::new().enable_auto_start(),
                )
                .with_permission_mode(mode);

            // Create and connect client
            let mut client = IFlowClient::new(Some(options));

            println!(
                "🔗 Connecting to iFlow via WebSocket with {} permission mode...",
                mode_name
            );
            client.connect().await?;
            println!("✅ Connected to iFlow via WebSocket");

            // Receive and process responses
            println!("📥 Receiving responses...");
            let mut message_stream = client.messages();

            let message_task = tokio::task::spawn_local(async move {
                let mut stdout = std::io::stdout();

                while let Some(message) = message_stream.next().await {
                    match message {
                        Message::Assistant { content } => {
                            print!("{}", content);
                            stdout
                                .flush()
                                .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })?;
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
                        Message::Error {
                            code,
                            message: msg,
                            details: _,
                        } => {
                            eprintln!("\n❌ Error {}: {}", code, msg);
                            break;
                        }
                        Message::User { content } => {
                            println!("\n👤 User message: {}", content);
                        }
                    }
                }

                Ok::<(), Box<dyn std::error::Error>>(())
            });

            // Send a message that might trigger tool calls
            // For Auto mode keep a simple prompt. For Selective/Manual modes send a compound
            // instruction containing 7 independent operations and explicitly ask the assistant
            // to request permission before executing each operation. This is intended to
            // trigger 7 separate permission requests when permission mode is not Auto.
            let prompt = match mode {
                PermissionMode::Auto => {
                    // Build a compound instruction with 7 independent steps.
                    // Each step explicitly asks the assistant to request permission before executing.
                    let steps = vec![
                        "1) 列出当前目录的文件",
                        "2) 读取文件 /etc/hosts 的前 10 行",
                        "3) 创建一个名为 example_1.txt 的文件，写入 'Step 3'",
                        "4) 创建一个名为 example_2.txt 的文件，写入 'Step 4'",
                        "5) 在当前目录创建一个名为 dir_example 的目录",
                        "6) 在 dir_example 中创建一个名为 nested.txt 的文件，写入 'Nested'",
                        "7) 删除上面创建的 example_1.txt（如果存在）",
                        "8) 使用 find 查找文件 a 是否存在",
                        "9) 执行 git status 命令",
                        "10) 执行 file /etc/hosts 命令",
                    ];

                    let mut prompt = String::from("请按顺序执行下面 9=10 个独立操作。\n");
                    for s in steps {
                        prompt.push_str(s);
                        prompt.push_str("\n");
                    }
                    prompt.push_str("");
                    prompt
                },
                _ =>
                    "列出当前目录的文件，并创建一个名为 test.txt 的文件，内容为 'Hello, iFlow!'".to_string()
            };
            println!("📤 Sending: {}", prompt);
            
            // Handle the send_message result to catch timeout errors
            match client.send_message(&prompt, None).await {
                Ok(()) => {
                    println!("✅ Message sent successfully");
                }
                Err(IFlowError::Timeout(msg)) => {
                    eprintln!("⏰ Timeout error occurred: {}", msg);
                    eprintln!("This may be due to processing delays.");
                    eprintln!("Consider increasing the timeout or checking the iFlow process.");
                }
                Err(e) => {
                    eprintln!("❌ Error sending message: {}", e);
                    return Err(e.into());
                }
            }

            // Wait for the message handling task to finish with a timeout
            match tokio::time::timeout(std::time::Duration::from_secs(60), message_task).await {
                Ok(Ok(Ok(()))) => {
                    println!("✅ Message handling completed successfully");
                }
                Ok(Ok(Err(err))) => {
                    eprintln!("❌ Error in message handling: {}", err);
                    return Err(err);
                }
                Ok(Err(err)) => {
                    eprintln!("❌ Message task panicked: {}", err);
                    return Err(Box::new(err));
                }
                Err(_) => {
                    println!("⏰ Timeout waiting for message handling to complete");
                }
            }

            // Disconnect
            println!("\n🔌 Disconnecting...");
            client.disconnect().await?;
            println!("👋 Disconnected from iFlow");

            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .await?;

    Ok(())
}