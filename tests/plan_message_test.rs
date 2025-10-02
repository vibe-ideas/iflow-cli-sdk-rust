//! Tests for PlanMessage functionality
use iflow_cli_sdk_rust::types::{PlanEntry, PlanPriority, PlanStatus};

#[tokio::test]
async fn test_plan_message_structure() {
    // Create a PlanEntry instance
    let entry = PlanEntry {
        content: "Test plan entry".to_string(),
        priority: PlanPriority::High,
        status: PlanStatus::InProgress,
    };

    // Verify the entry has the expected values
    assert_eq!(entry.content, "Test plan entry");
    assert_eq!(entry.priority, PlanPriority::High);
    assert_eq!(entry.status, PlanStatus::InProgress);
}

#[tokio::test]
async fn test_plan_message_default_values() {
    // Create a PlanEntry with default values
    let entry = PlanEntry::default();

    // Verify the default values
    assert_eq!(entry.content, "");
    assert_eq!(entry.priority, PlanPriority::Medium);
    assert_eq!(entry.status, PlanStatus::Pending);
}

#[tokio::test]
async fn test_plan_priority_variants() {
    // Test all PlanPriority variants
    assert_eq!(PlanPriority::High, PlanPriority::High);
    assert_eq!(PlanPriority::Medium, PlanPriority::Medium);
    assert_eq!(PlanPriority::Low, PlanPriority::Low);
}

#[tokio::test]
async fn test_plan_status_variants() {
    // Test all PlanStatus variants
    assert_eq!(PlanStatus::Pending, PlanStatus::Pending);
    assert_eq!(PlanStatus::InProgress, PlanStatus::InProgress);
    assert_eq!(PlanStatus::Completed, PlanStatus::Completed);
}