# iFlow CLI Rust SDK

This directory contains the iFlow CLI SDK for Rust, which allows developers to interact with iFlow using the Agent Communication Protocol (ACP).

## Project Overview

The iFlow CLI SDK for Rust provides a powerful interface for communicating with iFlow. It offers both simple query functions and a full bidirectional client for complex interactions. The SDK automatically manages the iFlow process lifecycle and supports both stdio and WebSocket communication.

### Key Features

- **Automatic Process Management**: SDK automatically starts and manages iFlow process
- **Stdio Communication**: Communicate with iFlow via stdio
- **WebSocket Communication**: Communicate with iFlow via WebSocket for better performance and reliability
- **Bidirectional Communication**: Real-time streaming messages and responses
- **Tool Call Management**: Fine-grained permission control for tool execution
- **Task Planning**: Receive and process structured task plans
- **Raw Data Access**: Debug and inspect protocol-level messages
- **Async/Await Support**: Modern async Rust with full type safety

## Building and Running

### Prerequisites

- Rust 1.70+
- iFlow CLI installed with `--experimental-acp` support (or use auto-start feature)

### Building

```bash
cargo build
```

### Testing

```bash
cargo test

# e2e tests
cargo test --test e2e_tests -- --nocapture
```

### Running Examples

```bash
# Simple query example
cargo run --example query

# Simple query with custom configuration example
cargo run --example query_with_config

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

### Running with logging

```bash
RUST_LOG=debug cargo run --example basic_client
```

## Development Conventions

- The SDK is organized into several modules:
  - `client` - Main IFlowClient implementation with stdio communication
  - `types` - Type definitions and message structures
  - `process_manager` - iFlow process lifecycle management
  - `query` - Convenience functions for simple queries
  - `error` - Error types and handling
  - `logger` - Message logging functionality
  - `websocket_transport` - Low-level WebSocket communication
  - `acp_protocol` - Implementation of the Agent Communication Protocol

- Uses the official `agent-client-protocol` crate for ACP implementation
- Follows Rust async/await patterns with Tokio runtime
- Provides both high-level convenience functions and low-level client control
- WebSocketConfig provides default parameters for reconnect_attempts and reconnect_interval, with auto-start mode support
