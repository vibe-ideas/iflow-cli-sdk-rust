//! Unit tests for MCP (Model Context Protocol) support in iFlow SDK
//!
//! These tests verify the correct behavior of MCP server configuration
//! and integration with iFlow SDK.
//!
//! Note: Currently only Stdio MCP servers are supported in agent-client-protocol v0.4.5.
//! HTTP and SSE variants may be added in future versions.

#[cfg(test)]
mod tests {
    use iflow_cli_sdk_rust::types::{LoggingConfig, PermissionMode};
    use iflow_cli_sdk_rust::{EnvVariable, IFlowOptions, McpServer};
    use std::path::PathBuf;

    /// Test creating MCP server configuration
    #[test]
    fn test_mcp_server_creation() {
        // Test Stdio MCP server configuration
        let mcp_server = McpServer::Stdio {
            name: "filesystem".to_string(),
            command: PathBuf::from("mcp-server-filesystem"),
            args: vec![
                "--allowed-dirs".to_string(),
                ".".to_string(),
                "--verbose".to_string(),
            ],
            env: vec![
                EnvVariable {
                    name: "DEBUG".to_string(),
                    value: "1".to_string(),
                    meta: None,
                },
                EnvVariable {
                    name: "LOG_LEVEL".to_string(),
                    value: "info".to_string(),
                    meta: None,
                },
            ],
        };

        // Verify the configuration
        match &mcp_server {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "filesystem");
                assert_eq!(command, &PathBuf::from("mcp-server-filesystem"));
                assert_eq!(args.len(), 3);
                assert_eq!(args[0], "--allowed-dirs");
                assert_eq!(args[1], ".");
                assert_eq!(args[2], "--verbose");
                assert_eq!(env.len(), 2);
                assert_eq!(env[0].name, "DEBUG");
                assert_eq!(env[0].value, "1");
                assert_eq!(env[1].name, "LOG_LEVEL");
                assert_eq!(env[1].value, "info");
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test IFlowOptions with MCP servers configuration
    #[test]
    fn test_iflow_options_with_mcp_servers() {
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
                args: vec!["--allowed-commands".to_string(), "ls,cat,echo".to_string()],
                env: vec![],
            },
        ];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers.clone())
            .with_auto_start(false)
            .with_permission_mode(PermissionMode::Manual);

        // Verify the options
        assert_eq!(options.mcp_servers.len(), 2);
        assert!(!options.process.auto_start);
        assert_eq!(options.permission_mode, PermissionMode::Manual);

        // Verify first MCP server
        match &options.mcp_servers[0] {
            McpServer::Stdio { name, command, .. } => {
                assert_eq!(name, "filesystem");
                assert_eq!(command, &PathBuf::from("mcp-server-filesystem"));
            }
            _ => panic!("Expected Stdio MCP server"),
        }

        // Verify second MCP server
        match &options.mcp_servers[1] {
            McpServer::Stdio { name, command, .. } => {
                assert_eq!(name, "command");
                assert_eq!(command, &PathBuf::from("mcp-server-command"));
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test MCP server environment variables
    #[test]
    fn test_mcp_server_env_variables() {
        let env_vars = vec![
            EnvVariable {
                name: "CUSTOM_VAR".to_string(),
                value: "custom_value".to_string(),
                meta: Some(serde_json::json!({"description": "Custom environment variable"})),
            },
            EnvVariable {
                name: "ANOTHER_VAR".to_string(),
                value: "another_value".to_string(),
                meta: None,
            },
        ];

        let mcp_server = McpServer::Stdio {
            name: "test-server".to_string(),
            command: PathBuf::from("test-server"),
            args: vec![],
            env: env_vars,
        };

        match &mcp_server {
            McpServer::Stdio { env, .. } => {
                assert_eq!(env.len(), 2);
                assert_eq!(env[0].name, "CUSTOM_VAR");
                assert_eq!(env[0].value, "custom_value");
                assert!(env[0].meta.is_some());

                assert_eq!(env[1].name, "ANOTHER_VAR");
                assert_eq!(env[1].value, "another_value");
                assert!(env[1].meta.is_none());
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test MCP server with complex arguments
    #[test]
    fn test_mcp_server_complex_args() {
        let mcp_server = McpServer::Stdio {
            name: "complex-server".to_string(),
            command: PathBuf::from("/usr/local/bin/complex-mcp-server"),
            args: vec![
                "--config".to_string(),
                "/path/to/config.json".to_string(),
                "--log-level".to_string(),
                "debug".to_string(),
                "--max-connections".to_string(),
                "10".to_string(),
            ],
            env: vec![EnvVariable {
                name: "RUST_LOG".to_string(),
                value: "debug".to_string(),
                meta: None,
            }],
        };

        match &mcp_server {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "complex-server");
                assert_eq!(command, &PathBuf::from("/usr/local/bin/complex-mcp-server"));
                assert_eq!(args.len(), 6);
                assert_eq!(args[0], "--config");
                assert_eq!(args[1], "/path/to/config.json");
                assert_eq!(args[2], "--log-level");
                assert_eq!(args[3], "debug");
                assert_eq!(args[4], "--max-connections");
                assert_eq!(args[5], "10");
                assert_eq!(env.len(), 1);
                assert_eq!(env[0].name, "RUST_LOG");
                assert_eq!(env[0].value, "debug");
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test IFlowOptions with MCP servers and other configurations
    #[test]
    fn test_iflow_options_comprehensive_mcp() {
        let mcp_servers = vec![McpServer::Stdio {
            name: "filesystem".to_string(),
            command: PathBuf::from("mcp-server-filesystem"),
            args: vec![
                "--allowed-dirs".to_string(),
                "/home/user".to_string(),
                "/tmp".to_string(),
            ],
            env: vec![],
        }];

        let options = IFlowOptions::new()
            .with_mcp_servers(mcp_servers)
            .with_timeout(60.0)
            .with_auto_start(true)
            .with_permission_mode(PermissionMode::Selective)
            .with_logging_config(LoggingConfig {
                enabled: true,
                level: "DEBUG".to_string(),
                logger_config: iflow_cli_sdk_rust::logger::LoggerConfig {
                    enabled: true,
                    log_file: PathBuf::from("logs/mcp_test.log"),
                    max_file_size: 5 * 1024 * 1024, // 5MB
                    max_files: 3,
                },
            });

        // Verify all configurations
        assert_eq!(options.mcp_servers.len(), 1);
        assert_eq!(options.timeout, 60.0);
        assert!(options.process.auto_start);
        assert_eq!(options.permission_mode, PermissionMode::Selective);
        assert!(options.logging.enabled);
        assert_eq!(options.logging.level, "DEBUG");
        assert!(options.logging.logger_config.enabled);
        assert_eq!(
            options.logging.logger_config.log_file,
            PathBuf::from("logs/mcp_test.log")
        );
        assert_eq!(options.logging.logger_config.max_file_size, 5 * 1024 * 1024);
        assert_eq!(options.logging.logger_config.max_files, 3);
    }

    /// Test serialization of MCP server configuration
    #[test]
    fn test_mcp_server_serialization() {
        use serde_json;

        let mcp_server = McpServer::Stdio {
            name: "test-serialization".to_string(),
            command: PathBuf::from("test-server"),
            args: vec!["--test".to_string()],
            env: vec![EnvVariable {
                name: "TEST_VAR".to_string(),
                value: "test_value".to_string(),
                meta: None,
            }],
        };

        // Serialize to JSON
        let serialized = serde_json::to_string(&mcp_server).unwrap();

        // Verify the JSON structure contains expected fields
        assert!(serialized.contains("\"name\":\"test-serialization\""));
        assert!(serialized.contains("\"command\":\"test-server\""));
        assert!(serialized.contains("\"args\":[\"--test\"]"));
        assert!(serialized.contains("\"env\":"));
        assert!(serialized.contains("\"TEST_VAR\""));
        assert!(serialized.contains("\"test_value\""));
    }

    /// Test placeholder for future HTTP MCP server support
    /// This test will be updated when HTTP variant is available
    #[test]
    fn test_http_mcp_server_placeholder() {
        // Note: HTTP MCP server variant is not yet available in agent-client-protocol v0.4.5
        // This test serves as a placeholder for when HTTP support is added
        //
        // Example of what HTTP MCP server configuration might look like:
        // let http_server = McpServer::Http {
        //     name: "http-server".to_string(),
        //     url: "http://localhost:8080".to_string(),
        // };
        //
        // For now, we just verify that Stdio variant works correctly
        let stdio_server = McpServer::Stdio {
            name: "stdio-server".to_string(),
            command: PathBuf::from("test-server"),
            args: vec!["--http".to_string()],
            env: vec![],
        };

        match stdio_server {
            McpServer::Stdio { name, .. } => {
                assert_eq!(name, "stdio-server");
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test placeholder for future SSE MCP server support
    /// This test will be updated when SSE variant is available
    #[test]
    fn test_sse_mcp_server_placeholder() {
        // Note: SSE MCP server variant is not yet available in agent-client-protocol v0.4.5
        // This test serves as a placeholder for when SSE support is added
        //
        // Example of what SSE MCP server configuration might look like:
        // let sse_server = McpServer::Sse {
        //     name: "sse-server".to_string(),
        //     url: "http://localhost:8081".to_string(),
        // };
        //
        // For now, we just verify that Stdio variant works correctly
        let stdio_server = McpServer::Stdio {
            name: "stdio-server".to_string(),
            command: PathBuf::from("test-server"),
            args: vec!["--sse".to_string()],
            env: vec![],
        };

        match stdio_server {
            McpServer::Stdio { name, .. } => {
                assert_eq!(name, "stdio-server");
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test MCP server with empty configuration
    #[test]
    fn test_mcp_server_empty_config() {
        let mcp_server = McpServer::Stdio {
            name: "empty-server".to_string(),
            command: PathBuf::from("empty-server"),
            args: vec![],
            env: vec![],
        };

        match &mcp_server {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "empty-server");
                assert_eq!(command, &PathBuf::from("empty-server"));
                assert!(args.is_empty());
                assert!(env.is_empty());
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }

    /// Test multiple MCP servers with different configurations
    #[test]
    fn test_multiple_mcp_servers() {
        let mcp_servers = vec![
            McpServer::Stdio {
                name: "server1".to_string(),
                command: PathBuf::from("server1"),
                args: vec!["--flag1".to_string()],
                env: vec![EnvVariable {
                    name: "VAR1".to_string(),
                    value: "value1".to_string(),
                    meta: None,
                }],
            },
            McpServer::Stdio {
                name: "server2".to_string(),
                command: PathBuf::from("server2"),
                args: vec!["--flag2".to_string(), "arg2".to_string()],
                env: vec![],
            },
            McpServer::Stdio {
                name: "server3".to_string(),
                command: PathBuf::from("server3"),
                args: vec![
                    "--flag3".to_string(),
                    "arg3".to_string(),
                    "--verbose".to_string(),
                ],
                env: vec![
                    EnvVariable {
                        name: "VAR3A".to_string(),
                        value: "value3a".to_string(),
                        meta: None,
                    },
                    EnvVariable {
                        name: "VAR3B".to_string(),
                        value: "value3b".to_string(),
                        meta: None,
                    },
                ],
            },
        ];

        let options = IFlowOptions::new().with_mcp_servers(mcp_servers);

        assert_eq!(options.mcp_servers.len(), 3);

        // Verify each server
        match &options.mcp_servers[0] {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "server1");
                assert_eq!(command, &PathBuf::from("server1"));
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], "--flag1");
                assert_eq!(env.len(), 1);
            }
            _ => panic!("Expected Stdio MCP server"),
        }

        match &options.mcp_servers[1] {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "server2");
                assert_eq!(command, &PathBuf::from("server2"));
                assert_eq!(args.len(), 2);
                assert_eq!(args[0], "--flag2");
                assert_eq!(args[1], "arg2");
                assert!(env.is_empty());
            }
            _ => panic!("Expected Stdio MCP server"),
        }

        match &options.mcp_servers[2] {
            McpServer::Stdio {
                name,
                command,
                args,
                env,
            } => {
                assert_eq!(name, "server3");
                assert_eq!(command, &PathBuf::from("server3"));
                assert_eq!(args.len(), 3);
                assert_eq!(args[0], "--flag3");
                assert_eq!(args[1], "arg3");
                assert_eq!(args[2], "--verbose");
                assert_eq!(env.len(), 2);
            }
            _ => panic!("Expected Stdio MCP server"),
        }
    }
}
