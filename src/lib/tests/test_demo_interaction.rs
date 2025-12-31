//! Integration Tests for Demo Interaction
//! Tests verifying counter demo increments and state changes

// Note: Full interaction tests require the Slint runtime
// These tests verify the data structures and state management logic

use slint_showcase_lib::data::capability_demo::{CapabilityDemo, Difficulty};

#[test]
fn test_counter_demo_exists() {
    let demos = slint_showcase_lib::data::get_all_demos();
    let counter = demos.iter().find(|d| d.id == "counter");
    assert!(counter.is_some(), "Counter demo should exist");

    let counter = counter.unwrap();
    assert_eq!(counter.id, "counter");
    assert_eq!(counter.title, "Counter");
    assert_eq!(counter.category, "interactive");
    assert_eq!(counter.difficulty, Difficulty::Beginner);
}

#[test]
fn test_demo_parameters_structure() {
    // Verify demo parameters can be set and accessed
    let mut demo = CapabilityDemo::new(
        "test-demo",
        "Test Demo",
        "A test demonstration",
        "interactive",
        "demos/test.slint",
    );

    // Set difficulty
    demo = demo.with_difficulty(Difficulty::Intermediate);
    assert_eq!(demo.difficulty, Difficulty::Intermediate);

    // Set estimated time
    demo = demo.with_estimated_time(5);
    assert_eq!(demo.estimated_time, 5);

    // Parameters should be optional
    assert!(demo.parameters.is_none());
}

#[test]
fn test_demo_validation_empty_id() {
    let demo = CapabilityDemo::new("", "Test", "Description", "interactive", "test.slint");

    assert!(demo.validate().is_err());
}

#[test]
fn test_demo_validation_empty_title() {
    let demo = CapabilityDemo::new("test-id", "", "Description", "interactive", "test.slint");

    assert!(demo.validate().is_err());
}

#[test]
fn test_demo_validation_valid() {
    let demo = CapabilityDemo::new(
        "valid-demo",
        "Valid Demo",
        "A valid demonstration with proper fields",
        "interactive",
        "demos/valid.slint",
    );

    assert!(demo.validate().is_ok());
}

#[test]
fn test_demo_validation_description_too_long() {
    // Create a description longer than 500 characters
    let long_desc = "x".repeat(501);
    let demo = CapabilityDemo::new(
        "long-demo",
        "Long Demo",
        long_desc,
        "interactive",
        "demos/long.slint",
    );

    assert!(demo.validate().is_err());
}

#[test]
fn test_demo_validation_id_too_long() {
    // Create an ID longer than 50 characters
    let long_id = "x".repeat(51);
    let demo = CapabilityDemo::new(
        long_id,
        "Long ID Demo",
        "Description",
        "interactive",
        "demos/long-id.slint",
    );

    assert!(demo.validate().is_err());
}
