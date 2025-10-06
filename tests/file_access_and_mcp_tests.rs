//! Additional tests for uncovered IFlowOptions methods and FileAccessConfig
//!
//! These tests target specific uncovered code paths

use iflow_cli_sdk_rust::{IFlowOptions, FileAccessConfig};
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_iflow_options_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), serde_json::json!("value1"));
    metadata.insert("key2".to_string(), serde_json::json!(42));
    
    let options = IFlowOptions::new().with_metadata(metadata.clone());
    assert_eq!(options.metadata.len(), 2);
    assert_eq!(options.metadata.get("key1"), Some(&serde_json::json!("value1")));
    assert_eq!(options.metadata.get("key2"), Some(&serde_json::json!(42)));
}

#[test]
fn test_iflow_options_with_file_access_config() {
    let file_config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![PathBuf::from("/tmp"), PathBuf::from("/home")]),
        read_only: true,
        max_size: 5 * 1024 * 1024,
    };
    
    let options = IFlowOptions::new().with_file_access_config(file_config.clone());
    assert_eq!(options.file_access.enabled, true);
    assert_eq!(options.file_access.read_only, true);
    assert_eq!(options.file_access.max_size, 5 * 1024 * 1024);
}

#[test]
fn test_file_access_config_default() {
    let config = FileAccessConfig::default();
    assert_eq!(config.enabled, false);
    assert_eq!(config.read_only, false);
    assert_eq!(config.max_size, 10 * 1024 * 1024);
    assert!(config.allowed_dirs.is_none());
}

#[test]
fn test_file_access_config_custom() {
    let config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![PathBuf::from("/data")]),
        read_only: false,
        max_size: 1024,
    };
    
    assert_eq!(config.enabled, true);
    assert_eq!(config.read_only, false);
    assert_eq!(config.max_size, 1024);
    assert_eq!(config.allowed_dirs.as_ref().unwrap().len(), 1);
}

#[test]
fn test_file_access_config_clone() {
    let original = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![PathBuf::from("/tmp")]),
        read_only: true,
        max_size: 2048,
    };
    
    let cloned = original.clone();
    assert_eq!(original.enabled, cloned.enabled);
    assert_eq!(original.read_only, cloned.read_only);
    assert_eq!(original.max_size, cloned.max_size);
}

#[test]
fn test_iflow_options_with_empty_metadata() {
    let options = IFlowOptions::new().with_metadata(HashMap::new());
    assert_eq!(options.metadata.len(), 0);
}

#[test]
fn test_file_access_config_with_multiple_dirs() {
    let dirs = vec![
        PathBuf::from("/home/user/documents"),
        PathBuf::from("/home/user/downloads"),
        PathBuf::from("/tmp"),
    ];
    
    let config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(dirs.clone()),
        read_only: false,
        max_size: 50 * 1024 * 1024,
    };
    
    assert_eq!(config.allowed_dirs.as_ref().unwrap().len(), 3);
    assert_eq!(config.max_size, 50 * 1024 * 1024);
}

#[test]
fn test_iflow_options_complete_configuration() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), serde_json::json!("1.0"));
    
    let file_config = FileAccessConfig {
        enabled: true,
        allowed_dirs: Some(vec![PathBuf::from("/data")]),
        read_only: true,
        max_size: 20 * 1024 * 1024,
    };
    
    let options = IFlowOptions::new()
        .with_cwd(PathBuf::from("/app"))
        .with_timeout(200.0)
        .with_metadata(metadata)
        .with_file_access_config(file_config);
    
    assert_eq!(options.cwd, PathBuf::from("/app"));
    assert_eq!(options.timeout, 200.0);
    assert_eq!(options.metadata.len(), 1);
    assert_eq!(options.file_access.enabled, true);
}

#[test]
fn test_file_access_config_debug() {
    let config = FileAccessConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("FileAccessConfig"));
}

#[test]
fn test_metadata_various_types() {
    let mut metadata = HashMap::new();
    metadata.insert("string".to_string(), serde_json::json!("test"));
    metadata.insert("number".to_string(), serde_json::json!(123));
    metadata.insert("bool".to_string(), serde_json::json!(true));
    metadata.insert("array".to_string(), serde_json::json!([1, 2, 3]));
    
    let options = IFlowOptions::new().with_metadata(metadata.clone());
    assert_eq!(options.metadata.len(), 4);
}

#[test]
fn test_file_access_config_disabled() {
    let config = FileAccessConfig {
        enabled: false,
        allowed_dirs: None,
        read_only: true,
        max_size: 1024,
    };
    
    assert_eq!(config.enabled, false);
    assert!(config.allowed_dirs.is_none());
}

