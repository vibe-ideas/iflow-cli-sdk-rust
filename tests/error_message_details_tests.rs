//! Tests for ErrorMessageDetails type
//!
//! These tests ensure ErrorMessageDetails is properly covered

use iflow_cli_sdk_rust::types::ErrorMessageDetails;
use std::collections::HashMap;

#[test]
fn test_error_message_details_new() {
    let details = ErrorMessageDetails::new(404, "Not found".to_string());
    assert_eq!(details.code, 404);
    assert_eq!(details.message, "Not found");
    assert!(details.details.is_none());
}

#[test]
fn test_error_message_details_with_details() {
    let mut extra_details = HashMap::new();
    extra_details.insert(
        "resource".to_string(),
        serde_json::Value::String("user".to_string()),
    );
    extra_details.insert(
        "id".to_string(),
        serde_json::Value::Number(123.into()),
    );

    let details = ErrorMessageDetails::with_details(
        500,
        "Internal error".to_string(),
        extra_details.clone(),
    );

    assert_eq!(details.code, 500);
    assert_eq!(details.message, "Internal error");
    assert!(details.details.is_some());
    assert_eq!(details.details.as_ref().unwrap().len(), 2);
}

#[test]
fn test_error_message_details_debug() {
    let details = ErrorMessageDetails::new(400, "Bad request".to_string());
    let debug_str = format!("{:?}", details);
    assert!(debug_str.contains("ErrorMessageDetails"));
    assert!(debug_str.contains("400"));
}

#[test]
fn test_error_message_details_clone() {
    let details1 = ErrorMessageDetails::new(403, "Forbidden".to_string());
    let details2 = details1.clone();
    assert_eq!(details1.code, details2.code);
    assert_eq!(details1.message, details2.message);
}

#[test]
fn test_error_message_details_serialization() {
    let details = ErrorMessageDetails::new(401, "Unauthorized".to_string());
    let json = serde_json::to_string(&details).unwrap();
    assert!(json.contains("401"));
    assert!(json.contains("Unauthorized"));
}

#[test]
fn test_error_message_details_deserialization() {
    let json = r#"{"code":500,"message":"Server error"}"#;
    let details: ErrorMessageDetails = serde_json::from_str(json).unwrap();
    assert_eq!(details.code, 500);
    assert_eq!(details.message, "Server error");
}

#[test]
fn test_error_message_details_with_complex_details() {
    let mut extra_details = HashMap::new();
    extra_details.insert(
        "nested".to_string(),
        serde_json::json!({
            "field1": "value1",
            "field2": 42
        }),
    );

    let details = ErrorMessageDetails::with_details(
        422,
        "Validation error".to_string(),
        extra_details,
    );

    assert_eq!(details.code, 422);
    assert_eq!(details.message, "Validation error");
    
    let nested_value = details.details.as_ref().unwrap().get("nested").unwrap();
    assert!(nested_value.is_object());
}

#[test]
fn test_error_message_details_serialization_skip_none() {
    let details = ErrorMessageDetails::new(200, "OK".to_string());
    let json = serde_json::to_string(&details).unwrap();
    // Details field should not be present when it's None
    assert!(!json.contains("details"));
}
