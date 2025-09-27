//! 测试消息流实时性

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试消息流实时性");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let options = IFlowOptions::new().with_auto_start_process(true);

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 连接到 iFlow...");
            client.connect().await?;
            println!("✅ 连接成功");

            // 发送一个简单的消息
            let prompt = "请说 'Hello World' 然后结束";
            println!("📤 发送消息: {}", prompt);
            client.send_message(prompt, None).await?;

            println!("📥 开始接收消息...");
            let start_time = Instant::now();
            let mut message_count = 0;

            let mut message_stream = client.messages();

            while let Some(message) = message_stream.next().await {
                message_count += 1;
                let elapsed = start_time.elapsed();

                match message {
                    Message::Assistant { content } => {
                        println!("[{:.2}s] 💬 Assistant: {}", elapsed.as_secs_f32(), content);
                    }
                    Message::ToolCall { id, name, status } => {
                        println!(
                            "[{:.2}s] 🔧 ToolCall: {} ({}): {}",
                            elapsed.as_secs_f32(),
                            id,
                            name,
                            status
                        );
                    }
                    Message::Plan { entries } => {
                        println!("[{:.2}s] 📋 Plan: {:?}", elapsed.as_secs_f32(), entries);
                    }
                    Message::TaskFinish { reason } => {
                        println!(
                            "[{:.2}s] ✅ TaskFinish: {:?}",
                            elapsed.as_secs_f32(),
                            reason
                        );
                        break;
                    }
                    Message::Error { code, message: msg } => {
                        println!("[{:.2}s] ❌ Error {}: {}", elapsed.as_secs_f32(), code, msg);
                    }
                    Message::User { content } => {
                        println!("[{:.2}s] 👤 User: {}", elapsed.as_secs_f32(), content);
                    }
                }

                // 刷新标准输出以确保实时显示
                std::io::stdout().flush()?;
            }

            println!(
                "📊 测试完成: 接收到 {} 条消息，耗时 {:.2} 秒",
                message_count,
                start_time.elapsed().as_secs_f32()
            );

            client.disconnect().await?;
            println!("👋 断开连接");

            Ok(())
        })
        .await
}
