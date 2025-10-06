//! Additional comprehensive tests for types module
//!
//! These tests improve coverage for types module

use iflow_cli_sdk_rust::types::{PermissionMode, PlanPriority, PlanStatus, ToolCallStatus};

#[test]
fn test_permission_mode_variants() {
    let auto = PermissionMode::Auto;
    let manual = PermissionMode::Manual;
    let selective = PermissionMode::Selective;

    assert_eq!(auto, PermissionMode::Auto);
    assert_eq!(manual, PermissionMode::Manual);
    assert_eq!(selective, PermissionMode::Selective);
}

#[test]
fn test_permission_mode_default() {
    let default = PermissionMode::default();
    assert_eq!(default, PermissionMode::Auto);
}

#[test]
fn test_permission_mode_debug() {
    let mode = PermissionMode::Auto;
    let debug_str = format!("{:?}", mode);
    assert!(debug_str.contains("Auto"));
}

#[test]
fn test_permission_mode_clone() {
    let mode1 = PermissionMode::Manual;
    let mode2 = mode1.clone();
    assert_eq!(mode1, mode2);
}

#[test]
fn test_permission_mode_copy() {
    let mode1 = PermissionMode::Selective;
    let mode2 = mode1; // Copy trait
    assert_eq!(mode1, mode2);
}

#[test]
fn test_permission_mode_equality() {
    assert_eq!(PermissionMode::Auto, PermissionMode::Auto);
    assert_ne!(PermissionMode::Auto, PermissionMode::Manual);
    assert_ne!(PermissionMode::Manual, PermissionMode::Selective);
}

#[test]
fn test_permission_mode_serialization() {
    let auto = PermissionMode::Auto;
    let json = serde_json::to_string(&auto).unwrap();
    assert_eq!(json, "\"auto\"");

    let manual = PermissionMode::Manual;
    let json = serde_json::to_string(&manual).unwrap();
    assert_eq!(json, "\"manual\"");

    let selective = PermissionMode::Selective;
    let json = serde_json::to_string(&selective).unwrap();
    assert_eq!(json, "\"selective\"");
}

#[test]
fn test_permission_mode_deserialization() {
    let auto: PermissionMode = serde_json::from_str("\"auto\"").unwrap();
    assert_eq!(auto, PermissionMode::Auto);

    let manual: PermissionMode = serde_json::from_str("\"manual\"").unwrap();
    assert_eq!(manual, PermissionMode::Manual);

    let selective: PermissionMode = serde_json::from_str("\"selective\"").unwrap();
    assert_eq!(selective, PermissionMode::Selective);
}

#[test]
fn test_tool_call_status_variants() {
    let pending = ToolCallStatus::Pending;
    let in_progress = ToolCallStatus::InProgress;
    let completed = ToolCallStatus::Completed;
    let failed = ToolCallStatus::Failed;
    let running = ToolCallStatus::Running;
    let finished = ToolCallStatus::Finished;
    let error = ToolCallStatus::Error;

    assert_eq!(pending, ToolCallStatus::Pending);
    assert_eq!(in_progress, ToolCallStatus::InProgress);
    assert_eq!(completed, ToolCallStatus::Completed);
    assert_eq!(failed, ToolCallStatus::Failed);
    assert_eq!(running, ToolCallStatus::Running);
    assert_eq!(finished, ToolCallStatus::Finished);
    assert_eq!(error, ToolCallStatus::Error);
}

#[test]
fn test_tool_call_status_debug() {
    let status = ToolCallStatus::Pending;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Pending"));
}

#[test]
fn test_tool_call_status_clone() {
    let status1 = ToolCallStatus::Completed;
    let status2 = status1.clone();
    assert_eq!(status1, status2);
}

#[test]
fn test_tool_call_status_equality() {
    assert_eq!(ToolCallStatus::Pending, ToolCallStatus::Pending);
    assert_ne!(ToolCallStatus::Pending, ToolCallStatus::Completed);
}

#[test]
fn test_tool_call_status_serialization() {
    let pending = ToolCallStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert_eq!(json, "\"pending\"");

    let in_progress = ToolCallStatus::InProgress;
    let json = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(json, "\"in_progress\"");

    let running = ToolCallStatus::Running;
    let json = serde_json::to_string(&running).unwrap();
    assert_eq!(json, "\"running\"");
}

#[test]
fn test_tool_call_status_deserialization() {
    let pending: ToolCallStatus = serde_json::from_str("\"pending\"").unwrap();
    assert_eq!(pending, ToolCallStatus::Pending);

    let in_progress: ToolCallStatus = serde_json::from_str("\"in_progress\"").unwrap();
    assert_eq!(in_progress, ToolCallStatus::InProgress);

    let running: ToolCallStatus = serde_json::from_str("\"running\"").unwrap();
    assert_eq!(running, ToolCallStatus::Running);
}

#[test]
fn test_plan_priority_variants() {
    let high = PlanPriority::High;
    let medium = PlanPriority::Medium;
    let low = PlanPriority::Low;

    assert_eq!(high, PlanPriority::High);
    assert_eq!(medium, PlanPriority::Medium);
    assert_eq!(low, PlanPriority::Low);
}

#[test]
fn test_plan_priority_default() {
    let default = PlanPriority::default();
    assert_eq!(default, PlanPriority::Medium);
}

#[test]
fn test_plan_priority_debug() {
    let priority = PlanPriority::High;
    let debug_str = format!("{:?}", priority);
    assert!(debug_str.contains("High"));
}

#[test]
fn test_plan_priority_clone() {
    let priority1 = PlanPriority::Low;
    let priority2 = priority1.clone();
    assert_eq!(priority1, priority2);
}

#[test]
fn test_plan_priority_equality() {
    assert_eq!(PlanPriority::High, PlanPriority::High);
    assert_ne!(PlanPriority::High, PlanPriority::Low);
}

#[test]
fn test_plan_priority_serialization() {
    let high = PlanPriority::High;
    let json = serde_json::to_string(&high).unwrap();
    assert_eq!(json, "\"high\"");

    let medium = PlanPriority::Medium;
    let json = serde_json::to_string(&medium).unwrap();
    assert_eq!(json, "\"medium\"");

    let low = PlanPriority::Low;
    let json = serde_json::to_string(&low).unwrap();
    assert_eq!(json, "\"low\"");
}

#[test]
fn test_plan_priority_deserialization() {
    let high: PlanPriority = serde_json::from_str("\"high\"").unwrap();
    assert_eq!(high, PlanPriority::High);

    let medium: PlanPriority = serde_json::from_str("\"medium\"").unwrap();
    assert_eq!(medium, PlanPriority::Medium);

    let low: PlanPriority = serde_json::from_str("\"low\"").unwrap();
    assert_eq!(low, PlanPriority::Low);
}

#[test]
fn test_plan_status_variants() {
    let pending = PlanStatus::Pending;
    let in_progress = PlanStatus::InProgress;
    let completed = PlanStatus::Completed;

    assert_eq!(pending, PlanStatus::Pending);
    assert_eq!(in_progress, PlanStatus::InProgress);
    assert_eq!(completed, PlanStatus::Completed);
}

#[test]
fn test_plan_status_default() {
    let default = PlanStatus::default();
    assert_eq!(default, PlanStatus::Pending);
}

#[test]
fn test_plan_status_debug() {
    let status = PlanStatus::Completed;
    let debug_str = format!("{:?}", status);
    assert!(debug_str.contains("Completed"));
}

#[test]
fn test_plan_status_clone() {
    let status1 = PlanStatus::InProgress;
    let status2 = status1.clone();
    assert_eq!(status1, status2);
}

#[test]
fn test_plan_status_equality() {
    assert_eq!(PlanStatus::Pending, PlanStatus::Pending);
    assert_ne!(PlanStatus::Pending, PlanStatus::Completed);
}

#[test]
fn test_plan_status_serialization() {
    let pending = PlanStatus::Pending;
    let json = serde_json::to_string(&pending).unwrap();
    assert_eq!(json, "\"pending\"");

    let in_progress = PlanStatus::InProgress;
    let json = serde_json::to_string(&in_progress).unwrap();
    assert_eq!(json, "\"in_progress\"");

    let completed = PlanStatus::Completed;
    let json = serde_json::to_string(&completed).unwrap();
    assert_eq!(json, "\"completed\"");
}

#[test]
fn test_plan_status_deserialization() {
    let pending: PlanStatus = serde_json::from_str("\"pending\"").unwrap();
    assert_eq!(pending, PlanStatus::Pending);

    let in_progress: PlanStatus = serde_json::from_str("\"in_progress\"").unwrap();
    assert_eq!(in_progress, PlanStatus::InProgress);

    let completed: PlanStatus = serde_json::from_str("\"completed\"").unwrap();
    assert_eq!(completed, PlanStatus::Completed);
}
