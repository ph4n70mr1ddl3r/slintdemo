//! Unit Tests for Catalog Data Structures
//! Tests verifying catalog data types and validation

use slint_showcase_lib::data::capability_demo::{CapabilityDemo, DemoValidationError, Difficulty};
use slint_showcase_lib::data::category::{Category, CategoryValidationError};

#[test]
fn test_category_new_creates_valid_category() {
    let category = Category::new(
        "test-category",
        "Test Category",
        "A test category description",
        1,
    );

    assert_eq!(category.id, "test-category");
    assert_eq!(category.name, "Test Category");
    assert_eq!(category.description, "A test category description");
    assert_eq!(category.display_order, 1);
    assert!(category.icon.is_none());
}

#[test]
fn test_category_with_icon() {
    let category =
        Category::new("icon-category", "Icon Category", "Category with icon", 2).with_icon("🎯");

    assert_eq!(category.icon, Some("🎯".to_string()));
}

#[test]
fn test_category_with_icon_replaces_previous() {
    let category = Category::new(
        "multi-icon",
        "Multi Icon",
        "Category with multiple icons",
        1,
    )
    .with_icon("A")
    .with_icon("B");

    assert_eq!(category.icon, Some("B".to_string()));
}

#[test]
fn test_category_validation_empty_id() {
    let category = Category::new("", "Name", "Description", 1);
    assert!(matches!(
        category.validate(),
        Err(CategoryValidationError::EmptyId)
    ));
}

#[test]
fn test_category_validation_id_too_long() {
    let long_id = "x".repeat(31);
    let category = Category::new(long_id, "Name", "Description", 1);
    assert!(matches!(
        category.validate(),
        Err(CategoryValidationError::IdTooLong)
    ));
}

#[test]
fn test_category_validation_empty_name() {
    let category = Category::new("id", "", "Description", 1);
    assert!(matches!(
        category.validate(),
        Err(CategoryValidationError::EmptyName)
    ));
}

#[test]
fn test_category_validation_empty_description() {
    let category = Category::new("id", "Name", "", 1);
    assert!(matches!(
        category.validate(),
        Err(CategoryValidationError::EmptyDescription)
    ));
}

#[test]
fn test_category_validation_valid() {
    let category = Category::new("valid-id", "Valid Name", "Valid Description", 1);
    assert!(category.validate().is_ok());
}

#[test]
fn test_category_clone_is_independent() {
    let original = Category::new("original", "Original", "Original desc", 1);
    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);
    assert_eq!(original.name, cloned.name);

    // Modify original - cloned should be unaffected
    let modified = original.with_icon("🌟");
    assert!(modified.icon.is_some());
    assert!(cloned.icon.is_none());
}

#[test]
fn test_demo_new_creates_valid_demo() {
    let demo = CapabilityDemo::new(
        "test-demo",
        "Test Demo",
        "A test demo description",
        "interactive",
        "demos/test.slint",
    );

    assert_eq!(demo.id, "test-demo");
    assert_eq!(demo.title, "Test Demo");
    assert_eq!(demo.description, "A test demo description");
    assert_eq!(demo.category, "interactive");
    assert_eq!(demo.slint_file, "demos/test.slint");
    assert_eq!(demo.difficulty, Difficulty::Beginner);
    assert_eq!(demo.estimated_time, 2);
    assert!(demo.parameters.is_none());
}

#[test]
fn test_demo_with_difficulty() {
    let demo = CapabilityDemo::new(
        "test-demo",
        "Test Demo",
        "Description",
        "interactive",
        "demos/test.slint",
    )
    .with_difficulty(Difficulty::Intermediate);

    assert_eq!(demo.difficulty, Difficulty::Intermediate);
}

#[test]
fn test_demo_with_estimated_time() {
    let demo = CapabilityDemo::new(
        "test-demo",
        "Test Demo",
        "Description",
        "interactive",
        "demos/test.slint",
    )
    .with_estimated_time(10);

    assert_eq!(demo.estimated_time, 10);
}

#[test]
fn test_demo_validation_empty_id() {
    let demo = CapabilityDemo::new("", "Title", "Description", "cat", "file.slint");
    assert!(matches!(demo.validate(), Err(DemoValidationError::EmptyId)));
}

#[test]
fn test_demo_validation_id_too_long() {
    let long_id = "x".repeat(51);
    let demo = CapabilityDemo::new(long_id, "Title", "Description", "cat", "file.slint");
    assert!(matches!(
        demo.validate(),
        Err(DemoValidationError::IdTooLong)
    ));
}

#[test]
fn test_demo_validation_empty_title() {
    let demo = CapabilityDemo::new("id", "", "Description", "cat", "file.slint");
    assert!(matches!(
        demo.validate(),
        Err(DemoValidationError::EmptyTitle)
    ));
}

#[test]
fn test_demo_validation_description_too_long() {
    let long_desc = "x".repeat(501);
    let demo = CapabilityDemo::new("id", "Title", long_desc, "cat", "file.slint");
    assert!(matches!(
        demo.validate(),
        Err(DemoValidationError::DescriptionTooLong)
    ));
}

#[test]
fn test_demo_validation_valid() {
    let demo = CapabilityDemo::new(
        "valid-id",
        "Valid Title",
        "Valid Description",
        "cat",
        "file.slint",
    );
    assert!(demo.validate().is_ok());
}

#[test]
fn test_difficulty_default_is_beginner() {
    let demo = CapabilityDemo::new("id", "Title", "Description", "cat", "file.slint");
    assert_eq!(demo.difficulty, Difficulty::Beginner);
}

#[test]
fn test_difficulty_ordering() {
    let beginner = Difficulty::Beginner;
    let intermediate = Difficulty::Intermediate;
    let advanced = Difficulty::Advanced;

    // Verify we can distinguish between difficulty levels
    assert_ne!(beginner, intermediate);
    assert_ne!(intermediate, advanced);
    assert_ne!(beginner, advanced);
}
