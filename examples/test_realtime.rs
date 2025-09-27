//! 测试实时消息接收

use futures::stream::StreamExt;
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试实时消息接收");

    // Use LocalSet for spawn_local compatibility
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let options = IFlowOptions::new().with_auto_start_process(true);

            let mut client = IFlowClient::new(Some(options));

            println!("🔗 连接到 iFlow...");
            client.connect().await?;
            println!("✅ 连接成功");

            let start_time = Instant::now();

            // 发送消息
            println!("📤 [{}s] 发送消息...", start_time.elapsed().as_secs_f32());
            client
                .send_message("请说 'Hello World' 然后结束", None)
                .await?;

            println!("📥 开始接收消息...");
            let mut message_stream = client.messages();
            let mut message_count = 0;

            while let Some(message) = message_stream.next().await {
                message_count += 1;
                let elapsed = start_time.elapsed().as_secs_f32();

                match message {
                    Message::Assistant { content } => {
                        println!("[{:.2}s] 💬 Assistant: {}", elapsed, content);
                        std::io::stdout().flush()?;
                    }
                    Message::ToolCall { id, name, status } => {
                        println!(
                            "[{:.2}s] 🔧 ToolCall: {} ({}): {}",
                            elapsed, id, name, status
                        );
                    }
                    Message::Plan { entries } => {
                        println!("[{:.2}s] 📋 Plan: {:?}", elapsed, entries);
                    }
                    Message::TaskFinish { reason } => {
                        println!("[{:.2}s] ✅ TaskFinish: {:?}", elapsed, reason);
                        break;
                    }
                    Message::Error { code, message: msg } => {
                        println!("[{:.2}s] ❌ Error {}: {}", elapsed, code, msg);
                        break;
                    }
                    Message::User { content } => {
                        println!("[{:.2}s] 👤 User: {}", elapsed, content);
                    }
                }
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
