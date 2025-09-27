# iFlow CLI SDK for Rust

A powerful Rust SDK for interacting with iFlow using the Agent Communication Protocol (ACP). This SDK provides both simple query functions and full bidirectional client for complex interactions.

## Features

- 🚀 **Automatic Process Management** - SDK automatically starts and manages iFlow process
- 🔌 **Stdio Communication** - Communicate with iFlow via stdio for better performance and reliability
- 🔄 **Bidirectional Communication** - Real-time streaming messages and responses
- 🛠️ **Tool Call Management** - Fine-grained permission control for tool execution
- 📋 **Task Planning** - Receive and process structured task plans
- 🔍 **Raw Data Access** - Debug and inspect protocol-level messages
- ⚡ **Async/Await Support** - Modern async Rust with full type safety

## TODO

- [ ] Add support for WebSocket communication
- [ ] 🤖 **Sub-agent Support** - Track and manage multiple AI agents via `agent_id`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iflow-cli-sdk-rust = "0.1.0"
```

Or install directly from the repository:

```bash
cargo add --git https://github.com/vibe-ideas/iflow-cli-sdk-rust
```

## Quick Start

### Simple Query

```rust
use iflow_cli_sdk_rust::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = query("What is 2 + 2?").await?;
    println!("{}", response); // "4"
    Ok(())
}
```

### Interactive Session

```rust
use iflow_cli_sdk_rust::{IFlowClient, IFlowOptions, Message};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = IFlowOptions::new()
        .with_auto_start_process(true);
    
    let mut client = IFlowClient::new(Some(options));
    client.connect().await?;
    
    client.send_message("Hello, iFlow!", None).await?;
    
    let mut message_stream = client.messages();
    while let Some(message) = message_stream.next().await {
        match message {
            Message::Assistant { content } => {
                print!("{}", content);
                std::io::stdout().flush()?;
            }
            Message::TaskFinish { .. } => {
                break;
            }
            _ => {
                // Handle other message types
            }
        }
    }
    
    client.disconnect().await?;
    Ok(())
}
```

### Streaming Responses

```rust
use iflow_cli_sdk_rust::query_stream;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = query_stream("Tell me a story").await?;
    
    while let Some(chunk) = stream.next().await {
        print!("{}", chunk);
        std::io::stdout().flush()?;
    }
    
    Ok(())
}
```

## Configuration

### Client Options

```rust
use iflow_cli_sdk_rust::IFlowOptions;

let options = IFlowOptions::new()
    .with_timeout(60.0)
    .with_file_access(true)
    .with_auto_start_process(true);
```

### Sandbox Mode

The SDK now uses stdio for communication with iFlow, so there's no need for WebSocket URLs. The `for_sandbox` method is no longer applicable.

## Message Types

The SDK handles various message types from iFlow:

- `Message::Assistant { content }` - AI assistant responses
- `Message::ToolCall { id, name, status }` - Tool execution requests
- `Message::Plan { entries }` - Structured task plans
- `Message::TaskFinish { reason }` - Task completion signals
- `Message::Error { code, message }` - Error notifications
- `Message::User { content }` - User message echoes

## Examples

Run the examples:

```bash
# Simple query example
cargo run --example query

# Interactive client example
cargo run --example basic_client

# Test response handling
cargo run --example test_response

# Explore API capabilities
cargo run --example explore_api

# Test real-time message streaming
cargo run --example test_stream

# Test real-time performance
cargo run --example test_realtime

# Logging example
cargo run --example logging_example
```

## Architecture

The SDK is organized into several modules:

- `client` - Main IFlowClient implementation with stdio communication
- `types` - Type definitions and message structures
- `process_manager` - iFlow process lifecycle management
- `query` - Convenience functions for simple queries
- `error` - Error types and handling
- `logger` - Message logging functionality

## Requirements

- Rust 1.70+
- iFlow CLI installed with `--experimental-acp` support (or use auto-start feature)

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running with logging

```bash
RUST_LOG=info cargo run --example basic_client
```

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
