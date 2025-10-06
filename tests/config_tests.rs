//! Comprehensive tests for configuration types
//!
//! These tests cover FileAccessConfig, ProcessConfig, LoggingConfig, and IFlowOptions

use iflow_cli_sdk_rust::types::{
    FileAccessConfig, ProcessConfig, LoggingConfig, IFlowOptions, WebSocketConfig,
};
use iflow_cli_sdk_rust::LoggerConfig;
use std::path::PathBuf;
use std::time::Duration;

#[test]
fn test_file_access_config_default() {
    let config = FileAccessConfig::default();
    assert_eq!(config.enabled, false);
    assert_eq!(config.allowed_dirs, None);
    assert_eq!(config.read_only, false);
    assert_eq!(config.max_size, 10 * 1024 * 1024);
}

#[test]
fn test_file_access_config_creation() {
    let config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![PathBuf::from("/tmp")]),
        read_only: true,
        max_size: 5 * 1024 * 1024,
    };
    assert_eq!(config.enabled, true);
    assert_eq!(config.allowed_dirs.as_ref().unwrap().len(), 1);
    assert_eq!(config.read_only, true);
    assert_eq!(config.max_size, 5 * 1024 * 1024);
}

#[test]
fn test_file_access_config_debug() {
    let config = FileAccessConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("FileAccessConfig"));
}

#[test]
fn test_file_access_config_clone() {
    let config1 = FileAccessConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.enabled, config2.enabled);
}

#[test]
fn test_process_config_default() {
    let config = ProcessConfig::default();
    assert_eq!(config.auto_start, true);
    assert_eq!(config.start_port, None);
}

#[test]
fn test_process_config_new() {
    let config = ProcessConfig::new();
    assert_eq!(config.auto_start, true);
    assert_eq!(config.start_port, None);
}

#[test]
fn test_process_config_auto_start() {
    let config = ProcessConfig::new().auto_start(false);
    assert_eq!(config.auto_start, false);
}

#[test]
fn test_process_config_start_port() {
    let config = ProcessConfig::new().start_port(8090);
    assert_eq!(config.start_port, Some(8090));
}

#[test]
fn test_process_config_manual_start() {
    let config = ProcessConfig::new().manual_start();
    assert_eq!(config.auto_start, false);
}

#[test]
fn test_process_config_enable_auto_start() {
    let config = ProcessConfig::new().manual_start().enable_auto_start();
    assert_eq!(config.auto_start, true);
}

#[test]
fn test_process_config_stdio_mode() {
    let config = ProcessConfig::new().start_port(8090).stdio_mode();
    assert_eq!(config.start_port, None);
}

#[test]
fn test_process_config_debug() {
    let config = ProcessConfig::new();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("ProcessConfig"));
}

#[test]
fn test_process_config_clone() {
    let config1 = ProcessConfig::new();
    let config2 = config1.clone();
    assert_eq!(config1.auto_start, config2.auto_start);
}

#[test]
fn test_process_config_chaining() {
    let config = ProcessConfig::new()
        .auto_start(false)
        .start_port(8080)
        .enable_auto_start();
    assert_eq!(config.auto_start, true);
    assert_eq!(config.start_port, Some(8080));
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();
    assert_eq!(config.enabled, false);
    assert_eq!(config.level, "INFO");
}

#[test]
fn test_logging_config_creation() {
    let logger_config = LoggerConfig {
        log_file: PathBuf::from("/tmp/test.log"),
        enabled: true,
        max_file_size: 1024,
        max_files: 3,
    };
    let config = LoggingConfig {
        enabled: true,
        level: "DEBUG".to_string(),
        logger_config: logger_config.clone(),
    };
    assert_eq!(config.enabled, true);
    assert_eq!(config.level, "DEBUG");
}

#[test]
fn test_logging_config_debug() {
    let config = LoggingConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("LoggingConfig"));
}

#[test]
fn test_logging_config_clone() {
    let config1 = LoggingConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.enabled, config2.enabled);
}

#[test]
fn test_iflow_options_default() {
    let options = IFlowOptions::default();
    assert_eq!(options.timeout, 120.0);
    assert_eq!(options.process.auto_start, true);
}

#[test]
fn test_iflow_options_new() {
    let options = IFlowOptions::new();
    assert_eq!(options.timeout, 120.0);
}

#[test]
fn test_iflow_options_with_timeout() {
    let options = IFlowOptions::new().with_timeout(60.0);
    assert_eq!(options.timeout, 60.0);
}

#[test]
fn test_iflow_options_with_auto_start() {
    let options = IFlowOptions::new().with_auto_start(false);
    assert_eq!(options.process.auto_start, false);
}

#[test]
fn test_iflow_options_with_process_config() {
    let process_config = ProcessConfig::new().start_port(9000);
    let options = IFlowOptions::new().with_process_config(process_config);
    assert_eq!(options.process.start_port, Some(9000));
}

#[test]
fn test_iflow_options_with_websocket_config() {
    let ws_config = WebSocketConfig::new("ws://localhost:8090/acp".to_string());
    let options = IFlowOptions::new().with_websocket_config(ws_config);
    assert!(options.websocket.is_some());
}

#[test]
fn test_iflow_options_debug() {
    let options = IFlowOptions::new();
    let debug_str = format!("{:?}", options);
    assert!(debug_str.contains("IFlowOptions"));
}

#[test]
fn test_iflow_options_clone() {
    let options1 = IFlowOptions::new();
    let options2 = options1.clone();
    assert_eq!(options1.timeout, options2.timeout);
}

#[test]
fn test_iflow_options_chaining() {
    let options = IFlowOptions::new()
        .with_timeout(90.0)
        .with_auto_start(false);
    assert_eq!(options.timeout, 90.0);
    assert_eq!(options.process.auto_start, false);
}

#[test]
fn test_websocket_config_with_reconnect_settings() {
    let config = WebSocketConfig::with_reconnect_settings(
        "ws://test:8080".to_string(),
        5,
        Duration::from_secs(10),
    );
    assert_eq!(config.url, Some("ws://test:8080".to_string()));
    assert_eq!(config.reconnect_attempts, 5);
    assert_eq!(config.reconnect_interval, Duration::from_secs(10));
}

#[test]
fn test_websocket_config_auto_start_with_reconnect_settings() {
    let config = WebSocketConfig::auto_start_with_reconnect_settings(
        10,
        Duration::from_secs(3),
    );
    assert_eq!(config.url, None);
    assert_eq!(config.reconnect_attempts, 10);
    assert_eq!(config.reconnect_interval, Duration::from_secs(3));
}

#[test]
fn test_websocket_config_debug() {
    let config = WebSocketConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("WebSocketConfig"));
}

#[test]
fn test_websocket_config_clone() {
    let config1 = WebSocketConfig::default();
    let config2 = config1.clone();
    assert_eq!(config1.reconnect_attempts, config2.reconnect_attempts);
}

#[test]
fn test_file_access_config_with_multiple_dirs() {
    let config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![
            PathBuf::from("/tmp"),
            PathBuf::from("/home"),
            PathBuf::from("/var"),
        ]),
        read_only: false,
        max_size: 20 * 1024 * 1024,
    };
    assert_eq!(config.allowed_dirs.as_ref().unwrap().len(), 3);
}

#[test]
fn test_iflow_options_with_cwd() {
    let options = IFlowOptions::new().with_cwd(PathBuf::from("/tmp"));
    assert_eq!(options.cwd, PathBuf::from("/tmp"));
}

#[test]
fn test_iflow_options_with_mcp_servers() {
    let options = IFlowOptions::new().with_mcp_servers(vec![]);
    assert_eq!(options.mcp_servers.len(), 0);
}

#[test]
fn test_iflow_options_with_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("key".to_string(), serde_json::Value::String("value".to_string()));
    let options = IFlowOptions::new().with_metadata(metadata.clone());
    assert_eq!(options.metadata.len(), 1);
}

#[test]
fn test_iflow_options_with_file_access_config() {
    let file_config = FileAccessConfig {
        enabled: true,
        allowed_dirs: None,
        read_only: true,
        max_size: 1024,
    };
    let options = IFlowOptions::new().with_file_access_config(file_config);
    assert_eq!(options.file_access.enabled, true);
}

#[test]
fn test_iflow_options_with_auth_method_id() {
    let options = IFlowOptions::new().with_auth_method_id("auth123".to_string());
    assert_eq!(options.auth_method_id, Some("auth123".to_string()));
}

#[test]
fn test_iflow_options_with_logging_config() {
    let logging_config = LoggingConfig {
        enabled: true,
        level: "DEBUG".to_string(),
        logger_config: LoggerConfig::default(),
    };
    let options = IFlowOptions::new().with_logging_config(logging_config);
    assert_eq!(options.logging.enabled, true);
}

#[test]
fn test_iflow_options_with_permission_mode() {
    use iflow_cli_sdk_rust::types::PermissionMode;
    let options = IFlowOptions::new().with_permission_mode(PermissionMode::Manual);
    assert_eq!(options.permission_mode, PermissionMode::Manual);
}

#[test]
fn test_iflow_options_complex_chaining() {
    use iflow_cli_sdk_rust::types::PermissionMode;
    let options = IFlowOptions::new()
        .with_timeout(30.0)
        .with_auto_start(false)
        .with_cwd(PathBuf::from("/custom"))
        .with_permission_mode(PermissionMode::Selective);
    
    assert_eq!(options.timeout, 30.0);
    assert_eq!(options.process.auto_start, false);
    assert_eq!(options.cwd, PathBuf::from("/custom"));
    assert_eq!(options.permission_mode, PermissionMode::Selective);
}
