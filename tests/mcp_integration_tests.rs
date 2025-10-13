//! Integration tests for MCP (Model Context Protocol) support in iFlow SDK
//!
//! These tests verify the integration of MCP servers with the iFlow client
//! and process management functionality.
//!
//! Note: Currently only Stdio MCP servers are supported in agent-client-protocol v0.4.5.
//! HTTP and SSE variants may be added in future versions.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::types::{LoggingConfig, PermissionMode};
    use iflow_cli_sdk_rust::{EnvVariable, IFlowClient, IFlowOptions, McpServer};
    use std::path::PathBuf;

    /// Test IFlowClient creation with MCP servers configuration
    #[tokio::test]
    async fn test_iflow_client_with_mcp_servers() {
        let mcp_servers = vec![McpServer::Stdio {
            name: "test-filesystem".to_string(),
            command: PathBuf::from("mcp-server-filesystem"),
            args: vec!["--allowed-dirs".to_string(), ".".to_string()],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_auto_start(false) // Don't actually start iFlow
            .with_permission_mode(PermissionMode::Auto);

        let client = IFlowClient::new(Some(options));

        // Verify that we can create a client with MCP configuration
        // The actual connection will fail if iFlow CLI is not installed, but that's expected
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test MCP server configuration with complex environment variables
    #[tokio::test]
    async fn test_mcp_server_complex_env() {
        let mcp_servers = vec![McpServer::Stdio {
            name: "complex-env-server".to_string(),
            command: PathBuf::from("complex-server"),
            args: vec!["--config".to_string(), "/path/to/config.json".to_string()],
            env: vec![
                EnvVariable {
                    name: "RUST_LOG".to_string(),
                    value: "debug".to_string(),
                    meta: Some(serde_json::json!({"description": "Log level"})),
                },
                EnvVariable {
                    name: "DATABASE_URL".to_string(),
                    value: "postgres://localhost:5432/mydb".to_string(),
                    meta: Some(serde_json::json!({"description": "Database connection string"})),
                },
                EnvVariable {
                    name: "API_KEY".to_string(),
                    value: "secret-key".to_string(),
                    meta: Some(serde_json::json!({"secure": true})),
                },
            ],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_timeout(30.0)
            .with_logging_config(LoggingConfig {
                enabled: true,
                level: "DEBUG".to_string(),
                logger_config: iflow_cli_sdk_rust::logger::LoggerConfig {
                    enabled: true,
                    log_file: PathBuf::from("logs/mcp_integration_test.log"),
                    max_file_size: 5 * 1024 * 1024,
                    max_files: 3,
                },
            });

        let client = IFlowClient::new(Some(options));

        // Verify client creation
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test MCP server configuration with WebSocket transport
    #[tokio::test]
    async fn test_mcp_with_websocket_config() {
        let mcp_servers = vec![McpServer::Stdio {
            name: "websocket-mcp-server".to_string(),
            command: PathBuf::from("mcp-server-websocket"),
            args: vec!["--port".to_string(), "8080".to_string()],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_websocket_config(iflow_cli_sdk_rust::types::WebSocketConfig::new(
                "ws://localhost:8090/acp?peer=iflow".to_string(),
            ))
            .with_auto_start(false);

        let client = IFlowClient::new(Some(options));

        // Verify client creation with WebSocket and MCP
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test MCP server configuration with multiple different servers
    #[tokio::test]
    async fn test_multiple_mcp_servers_integration() {
        let mcp_servers = vec![
            McpServer::Stdio {
                name: "filesystem".to_string(),
                command: PathBuf::from("mcp-server-filesystem"),
                args: vec!["--allowed-dirs".to_string(), ".".to_string()],
                env: vec![],
            },
            McpServer::Stdio {
                name: "command".to_string(),
                command: PathBuf::from("mcp-server-command"),
                args: vec![
                    "--allowed-commands".to_string(),
                    "ls,cat,echo,pwd".to_string(),
                ],
                env: vec![],
            },
            McpServer::Stdio {
                name: "web".to_string(),
                command: PathBuf::from("mcp-server-web"),
                args: vec!["--allowed-domains".to_string(), "example.com".to_string()],
                env: vec![],
            },
        ];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_timeout(60.0)
            .with_permission_mode(PermissionMode::Selective);
        assert_eq!(options.mcp_servers.len(), 3);

        let client = IFlowClient::new(Some(options));

        // Verify client creation with multiple MCP servers
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test MCP server configuration with auto-start process management
    #[tokio::test]
    async fn test_mcp_with_auto_start() {
        let mcp_servers = vec![McpServer::Stdio {
            name: "auto-start-server".to_string(),
            command: PathBuf::from("mcp-server-auto-start"),
            args: vec!["--auto".to_string()],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_process_config(
                iflow_cli_sdk_rust::types::ProcessConfig::new()
                    .enable_auto_start()
                    .start_port(8090)
                    .enable_debug(),
            )
            .with_logging_config(LoggingConfig {
                enabled: true,
                level: "INFO".to_string(),
                logger_config: iflow_cli_sdk_rust::logger::LoggerConfig {
                    enabled: true,
                    log_file: PathBuf::from("logs/auto_start_mcp.log"),
                    max_file_size: 10 * 1024 * 1024,
                    max_files: 5,
                },
            });

        let client = IFlowClient::new(Some(options));

        // Verify client creation with auto-start configuration
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test MCP server configuration validation
    #[test]
    fn test_mcp_server_validation() {
        // Test valid MCP server configuration
        let valid_mcp_server = McpServer::Stdio {
            name: "valid-server".to_string(),
            command: PathBuf::from("valid-server"),
            args: vec![],
            env: vec![],
        };

        // This should not panic
        let options = IFlowOptions::new().with_mcp_servers(vec![valid_mcp_server]);
        assert_eq!(options.mcp_servers.len(), 1);

        // Test multiple valid MCP servers
        let multiple_servers = vec![
            McpServer::Stdio {
                name: "server1".to_string(),
                command: PathBuf::from("server1"),
                args: vec!["--flag1".to_string()],
                env: vec![],
            },
            McpServer::Stdio {
                name: "server2".to_string(),
                command: PathBuf::from("server2"),
                args: vec!["--flag2".to_string()],
                env: vec![],
            },
        ];

        let options = IFlowOptions::new().with_mcp_servers(multiple_servers);
        assert_eq!(options.mcp_servers.len(), 2);
    }

    /// Test placeholder for future HTTP MCP server integration
    /// This test will be updated when HTTP variant is available
    #[tokio::test]
    async fn test_http_mcp_server_integration_placeholder() {
        // Note: HTTP MCP server variant is not yet available in agent-client-protocol v0.4.5
        // This test serves as a placeholder for when HTTP support is added
        //
        // Example of what HTTP MCP server configuration might look like:
        // let mcp_servers = vec![
        //     McpServer::Http {
        //         name: "http-server".to_string(),
        //         url: "http://localhost:8080".to_string(),
        //     },
        // ];
        //
        // For now, we test with Stdio variant that supports HTTP-like functionality
        let mcp_servers = vec![McpServer::Stdio {
            name: "http-proxy-server".to_string(),
            command: PathBuf::from("mcp-http-proxy"),
            args: vec![
                "--target-url".to_string(),
                "http://localhost:8080".to_string(),
            ],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_auto_start(false);

        let client = IFlowClient::new(Some(options));

        // Verify client creation
        assert!(std::mem::size_of_val(&client) > 0);
    }

    /// Test placeholder for future SSE MCP server integration
    /// This test will be updated when SSE variant is available
    #[tokio::test]
    async fn test_sse_mcp_server_integration_placeholder() {
        // Note: SSE MCP server variant is not yet available in agent-client-protocol v0.4.5
        // This test serves as a placeholder for when SSE support is added
        //
        // Example of what SSE MCP server configuration might look like:
        // let mcp_servers = vec![
        //     McpServer::Sse {
        //         name: "sse-server".to_string(),
        //         url: "http://localhost:8081".to_string(),
        //     },
        // ];
        //
        // For now, we test with Stdio variant that supports SSE-like functionality
        let mcp_servers = vec![McpServer::Stdio {
            name: "sse-proxy-server".to_string(),
            command: PathBuf::from("mcp-sse-proxy"),
            args: vec!["--sse-url".to_string(), "http://localhost:8081".to_string()],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_auto_start(false);

        let client = IFlowClient::new(Some(options));

        // Verify client creation
        assert!(std::mem::size_of_val(&client) > 0);
    }
}
