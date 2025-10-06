//! Tests for library-level exports and constants
//!
//! These tests ensure library constants and re-exports are working

use iflow_cli_sdk_rust::{VERSION, PROTOCOL_VERSION};

#[test]
fn test_version_constant() {
    assert!(!VERSION.is_empty());
    // Version should follow semver format
    let parts: Vec<&str> = VERSION.split('.').collect();
    assert!(parts.len() >= 2); // At least major.minor
}

#[test]
fn test_protocol_version() {
    assert_eq!(PROTOCOL_VERSION, 1);
}

#[test]
fn test_version_format() {
    // Version should contain only numbers and dots (and possibly pre-release info)
    assert!(VERSION.chars().next().unwrap().is_numeric() || VERSION.starts_with('0'));
}

#[test]
fn test_constants_are_consistent() {
    // Protocol version should be the same across modules
    assert_eq!(PROTOCOL_VERSION, iflow_cli_sdk_rust::types::PROTOCOL_VERSION);
}
