# iFlow CLI SDK Examples

This directory contains several examples demonstrating different ways to use the iFlow CLI SDK for Rust.

## Simple Query Examples

### `query.rs`

Demonstrates the simplest way to interact with iFlow using the `query` convenience function.

```bash
cargo run --example query
```

This example shows how to send a single query to iFlow and receive a complete response.

### `query_stream.rs`

Shows how to stream responses from iFlow in real-time as they are generated.

```bash
cargo run --example query_stream
```

This is useful for long responses where you want to show output as it's being generated.

## Bidirectional Communication Examples

### `basic_client.rs`

Demonstrates full bidirectional communication with iFlow, handling multiple message types.

```bash
cargo run --example basic_client
```

This example shows how to:

- Establish a connection with iFlow
- Send messages
- Handle different message types (Assistant, ToolCall, Plan, etc.)
- Process streaming responses

### `websocket_client.rs`

Shows how to use WebSocket communication instead of stdio.

```bash
cargo run --example websocket_client
```

This example demonstrates:

- Connecting to iFlow via WebSocket
- Configuring WebSocket-specific options
- Handling the same message types as the basic client

## Advanced Features Examples

### `permission_modes.rs`

Demonstrates how to control tool call permissions.

```bash
cargo run --example permission_modes
```

This example shows:

- Different permission modes (Auto, Manual, Selective)
- How to handle tool call requests
- Implementing custom permission logic

### `test_stream.rs`

Performance test for message streaming.

```bash
cargo run --example test_stream
```

This example:

- Measures message streaming performance
- Shows detailed timing information
- Demonstrates handling all message types with timestamps

### `test_response.rs`

Tests response handling with detailed output.

```bash
cargo run --example test_response
```

### `test_realtime.rs`

Tests real-time performance characteristics.

```bash
cargo run --example test_realtime
```

### `logging_example.rs`

Shows how to use the built-in logging functionality.

```bash
cargo run --example logging_example
```

This example demonstrates:

- Configuring logging options
- Writing logs to files
- Using different log levels

## Usage Tips

1. Make sure iFlow CLI is installed and available in your PATH, or use the auto-start feature
2. For WebSocket examples, ensure iFlow is running with WebSocket support
3. Most examples use `LocalSet` for proper async runtime compatibility
4. All examples follow the pattern of connecting, sending messages, handling responses, and disconnecting

## Running Examples

To run any example, use:

```bash
cargo run --example <example_name>
```

Replace `<example_name>` with the name of the example you want to run (without the `.rs` extension).
