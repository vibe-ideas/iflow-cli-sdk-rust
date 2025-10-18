# iFlow CLI SDK for Rust - Development Roadmap

This document outlines the current status and future development plans for the iFlow CLI Rust SDK. This roadmap is designed to guide development efforts to ensure the SDK is feature-complete and aligned with the [Python version](https://pypi.org/project/iflow-cli-sdk/).

## Current Status

The Rust SDK has implemented core functionality to meet basic iFlow interaction needs:

### Implemented Features

- ✅ **Basic Client Connection** - Supports connecting to iFlow via stdio and WebSocket
- ✅ **Automatic Process Management** - Automatically starts and stops the iFlow process
- ✅ **Message Sending and Receiving** - Supports sending text messages and receiving responses
- ✅ **Simple Query Interface** - Provides convenient `query` and `query_stream` functions
- ✅ **WebSocket Support** - Communicates with iFlow via WebSocket
- ✅ **Logging** - Supports message logging functionality
- ✅ **Permission Mode** - Supports different tool call permission modes

## Feature Gap Analysis

Compared to the Python SDK, the Rust SDK needs enhancement in the following areas:

### 1. Message Type Support

- Lacks complete support for `PlanMessage`
- Lacks detailed handling of `ToolCallMessage`
- Lacks handling of `UserMessage`
- Lacks detailed handling of `ErrorMessage`

### 2. Advanced Configuration Options

- Lacks support for MCP server configuration
- Lacks support for Hook configuration
- Lacks support for session settings
- Lacks complete support for authentication methods

### 3. Tool Call Management

- Lacks manual confirmation of tool calls
- Lacks tool call status tracking
- Lacks tool call content processing

### 4. File System Access

- Lacks complete support for file system access
- Lacks file permission control
- Lacks file size limit control

### 5. Session Management

- Lacks loading existing sessions
- Lacks session persistence support

## Development Plan

### Phase 1: Message Type Enhancement (v0.2.0)

**Goal**: Complete support for all ACP message types

- [X] Implement complete `PlanMessage` handling
- [X] Implement complete `ToolCallMessage` handling
- [X] Implement `UserMessage` handling
- [X] Implement detailed `ErrorMessage` handling
- [X] Add message type test cases

### Phase 2: Advanced Configuration Support (v0.3.0)

**Goal**: Provide configuration options comparable to the Python SDK

- [X] Implement MCP server configuration support
- [ ] Implement Hook configuration support
- [ ] Implement session settings support
- [ ] Implement complete authentication method support
- [ ] Add configuration option test cases

### Phase 3: Tool Call Enhancement (v0.4.0)

**Goal**: Provide complete tool call management functionality

- [ ] Implement manual confirmation of tool calls
- [ ] Implement tool call status tracking
- [ ] Implement tool call content processing
- [ ] Implement tool call rejection functionality
- [ ] Add tool call management test cases

### Phase 4: File System Access (v0.5.0)

**Goal**: Provide secure file system access functionality

- [ ] Implement file system access support
- [ ] Implement file permission control
- [ ] Implement file size limit control
- [ ] Add file access test cases
- [ ] Add security audit

### Phase 5: Session Management Enhancement (v0.6.0)

**Goal**: Provide complete session management functionality

- [ ] Implement loading existing sessions
- [ ] Implement session persistence support
- [ ] Add session management test cases

### Phase 6: Performance Optimization and Stability (v0.7.0)

**Goal**: Optimize SDK performance and improve stability

- [ ] Conduct performance benchmark testing
- [ ] Optimize memory usage
- [ ] Improve error handling robustness
- [ ] Add integration tests
- [ ] Add stress tests

### Phase 7: Documentation Enhancement (v0.8.0)

**Goal**: Provide complete documentation and examples

- [ ] Write complete API documentation
- [ ] Provide detailed usage examples
- [ ] Write migration guide
- [ ] Add frequently asked questions

## Long-term Goals

### Feature Completeness (v1.0.0)

- Achieve feature parity with the Python SDK
- Provide complete ACP protocol support
- Ensure API stability

### Ecosystem Integration

- Provide integration examples with popular Rust web frameworks
- Support async runtime selection
- Provide WASM support

## Contribution Guidelines

We welcome community contributions! If you'd like to participate in development:

1. Check GitHub issues for tasks to work on
2. Fork the repository and create a feature branch
3. Implement features and pass tests
4. Submit a Pull Request

Please ensure you follow the project's code style and testing requirements.
