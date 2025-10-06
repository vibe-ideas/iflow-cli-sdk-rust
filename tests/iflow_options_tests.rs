//! Additional comprehensive unit tests for IFlowOptions
//!
//! These tests cover all IFlowOptions methods and configurations

use iflow_cli_sdk_rust::{IFlowOptions, ProcessConfig, WebSocketConfig, LoggingConfig, LoggerConfig, PermissionMode};
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_iflow_options_default_values() {
    let options = IFlowOptions::default();
    
    assert_eq!(options.timeout, 120.0);
    assert_eq!(options.process.auto_start, true);
    assert_eq!(options.mcp_servers.len(), 0);
    assert_eq!(options.metadata.len(), 0);
    assert!(options.websocket.is_none());
    assert_eq!(options.permission_mode, PermissionMode::Auto);
}

#[test]
fn test_iflow_options_new() {
    let options = IFlowOptions::new();
    
    assert_eq!(options.timeout, 120.0);
    assert_eq!(options.process.auto_start, true);
}

#[test]
fn test_iflow_options_with_cwd() {
    let cwd = PathBuf::from("/tmp/test");
    let options = IFlowOptions::new().with_cwd(cwd.clone());
    
    assert_eq!(options.cwd, cwd);
}

#[test]
fn test_iflow_options_with_timeout() {
    let options = IFlowOptions::new().with_timeout(60.0);
    assert_eq!(options.timeout, 60.0);
}

#[test]
fn test_iflow_options_with_auto_start_true() {
    let options = IFlowOptions::new().with_auto_start(true);
    assert_eq!(options.process.auto_start, true);
}

#[test]
fn test_iflow_options_with_auto_start_false() {
    let options = IFlowOptions::new().with_auto_start(false);
    assert_eq!(options.process.auto_start, false);
}

#[test]
fn test_iflow_options_with_auth_method_id() {
    let options = IFlowOptions::new().with_auth_method_id("oauth2".to_string());
    assert_eq!(options.auth_method_id, Some("oauth2".to_string()));
}

#[test]
fn test_iflow_options_with_logging_config() {
    let logging_config = LoggingConfig {
        enabled: true,
        level: "DEBUG".to_string(),
        logger_config: LoggerConfig::default(),
    };
    
    let options = IFlowOptions::new().with_logging_config(logging_config.clone());
    assert_eq!(options.logging.enabled, true);
    assert_eq!(options.logging.level, "DEBUG");
}

#[test]
fn test_iflow_options_with_websocket_config() {
    let ws_config = WebSocketConfig::new("ws://localhost:9000/acp?peer=iflow".to_string());
    let options = IFlowOptions::new().with_websocket_config(ws_config);
    
    assert!(options.websocket.is_some());
    if let Some(ws) = options.websocket {
        assert_eq!(ws.url, Some("ws://localhost:9000/acp?peer=iflow".to_string()));
    }
}

#[test]
fn test_iflow_options_with_permission_mode() {
    let options = IFlowOptions::new().with_permission_mode(PermissionMode::Manual);
    assert_eq!(options.permission_mode, PermissionMode::Manual);
}

#[test]
fn test_iflow_options_multiple_chains() {
    let options = IFlowOptions::new()
        .with_timeout(90.0)
        .with_auto_start(false)
        .with_permission_mode(PermissionMode::Selective)
        .with_auth_method_id("custom".to_string());
    
    assert_eq!(options.timeout, 90.0);
    assert_eq!(options.process.auto_start, false);
    assert_eq!(options.permission_mode, PermissionMode::Selective);
    assert_eq!(options.auth_method_id, Some("custom".to_string()));
}

#[test]
fn test_iflow_options_process_config() {
    let process_config = ProcessConfig::new()
        .enable_auto_start()
        .start_port(9000);
    
    let options = IFlowOptions::new().with_process_config(process_config);
    assert_eq!(options.process.auto_start, true);
    assert_eq!(options.process.start_port, Some(9000));
}

#[test]
fn test_iflow_options_clone() {
    let original = IFlowOptions::new()
        .with_timeout(60.0)
        .with_auto_start(false);
    
    let cloned = original.clone();
    assert_eq!(original.timeout, cloned.timeout);
    assert_eq!(original.process.auto_start, cloned.process.auto_start);
}

#[test]
fn test_iflow_options_debug_format() {
    let options = IFlowOptions::new();
    let debug_str = format!("{:?}", options);
    assert!(debug_str.contains("IFlowOptions"));
}

#[test]
fn test_logging_config_default() {
    let config = LoggingConfig::default();
    assert_eq!(config.enabled, false);
    assert_eq!(config.level, "INFO");
}

#[test]
fn test_logging_config_clone() {
    let original = LoggingConfig::default();
    let cloned = original.clone();
    assert_eq!(original.enabled, cloned.enabled);
    assert_eq!(original.level, cloned.level);
}

#[test]
fn test_iflow_options_various_timeouts() {
    let options1 = IFlowOptions::new().with_timeout(30.0);
    let options2 = IFlowOptions::new().with_timeout(120.0);
    let options3 = IFlowOptions::new().with_timeout(300.0);
    
    assert_eq!(options1.timeout, 30.0);
    assert_eq!(options2.timeout, 120.0);
    assert_eq!(options3.timeout, 300.0);
}

#[test]
fn test_iflow_options_with_all_permission_modes() {
    let auto = IFlowOptions::new().with_permission_mode(PermissionMode::Auto);
    let manual = IFlowOptions::new().with_permission_mode(PermissionMode::Manual);
    let selective = IFlowOptions::new().with_permission_mode(PermissionMode::Selective);
    
    assert_eq!(auto.permission_mode, PermissionMode::Auto);
    assert_eq!(manual.permission_mode, PermissionMode::Manual);
    assert_eq!(selective.permission_mode, PermissionMode::Selective);
}

#[test]
fn test_iflow_options_websocket_none() {
    let options = IFlowOptions::new();
    assert!(options.websocket.is_none());
}

#[test]
fn test_iflow_options_websocket_some() {
    let options = IFlowOptions::new()
        .with_websocket_config(WebSocketConfig::auto_start());
    assert!(options.websocket.is_some());
}

#[test]
fn test_iflow_options_cwd_default() {
    let options = IFlowOptions::default();
    // cwd should be set to current directory or "."
    assert!(options.cwd.to_str().is_some());
}

#[test]
fn test_iflow_options_metadata_empty() {
    let options = IFlowOptions::new();
    assert_eq!(options.metadata.len(), 0);
}

#[test]
fn test_iflow_options_mcp_servers_empty() {
    let options = IFlowOptions::new();
    assert_eq!(options.mcp_servers.len(), 0);
}

#[test]
fn test_iflow_options_auth_method_none() {
    let options = IFlowOptions::new();
    assert!(options.auth_method_id.is_none());
}

#[test]
fn test_iflow_options_auth_method_some() {
    let options = IFlowOptions::new().with_auth_method_id("test".to_string());
    assert_eq!(options.auth_method_id, Some("test".to_string()));
}

#[test]
fn test_iflow_options_complete_chain() {
    let options = IFlowOptions::new()
        .with_cwd(PathBuf::from("/tmp"))
        .with_timeout(180.0)
        .with_auto_start(true)
        .with_auth_method_id("auth1".to_string())
        .with_permission_mode(PermissionMode::Manual)
        .with_websocket_config(WebSocketConfig::auto_start())
        .with_logging_config(LoggingConfig {
            enabled: true,
            level: "DEBUG".to_string(),
            logger_config: LoggerConfig::default(),
        });
    
    assert_eq!(options.cwd, PathBuf::from("/tmp"));
    assert_eq!(options.timeout, 180.0);
    assert_eq!(options.process.auto_start, true);
    assert_eq!(options.auth_method_id, Some("auth1".to_string()));
    assert_eq!(options.permission_mode, PermissionMode::Manual);
    assert!(options.websocket.is_some());
    assert_eq!(options.logging.enabled, true);
    assert_eq!(options.logging.level, "DEBUG");
}
