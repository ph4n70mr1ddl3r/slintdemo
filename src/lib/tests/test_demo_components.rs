//! Unit Tests for Demo Component State Management
//! Tests verifying demo component state and behavior logic

use slint_showcase_lib::data::capability_demo::{CapabilityDemo, Difficulty};
use slint_showcase_lib::data::category::{Category, CategoryValidationError};

#[test]
fn test_category_creation() {
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
fn test_get_showcase_categories() {
    let categories = slint_showcase_lib::data::load_categories();

    // Should have at least 4 categories
    assert!(categories.len() >= 4);

    // All categories should be valid
    for category in &categories {
        assert!(
            category.validate().is_ok(),
            "Category {} should be valid",
            category.id
        );
    }
}

#[test]
fn test_get_interactive_demos() {
    let demos = slint_showcase_lib::data::get_interactive_demos();

    // Should have at least 5 interactive demos
    assert!(demos.len() >= 5);

    // All demos should be in interactive category
    for demo in &demos {
        assert_eq!(demo.category, "interactive");
    }
}

#[test]
fn test_get_all_demos() {
    let demos = slint_showcase_lib::data::get_all_demos();

    // Should have more demos than just interactive
    assert!(demos.len() > 5);

    // All demos should have valid IDs
    for demo in &demos {
        assert!(!demo.id.is_empty());
        assert!(demo.id.len() <= 50);
    }
}

#[test]
fn test_find_demo() {
    let demo = slint_showcase_lib::data::find_demo("counter");
    assert!(demo.is_some());
    assert_eq!(demo.unwrap().id, "counter");
}

#[test]
fn test_find_demo_not_found() {
    let demo = slint_showcase_lib::data::find_demo("nonexistent");
    assert!(demo.is_none());
}

#[test]
fn test_find_category() {
    let category = slint_showcase_lib::data::find_category("interactive");
    assert!(category.is_some());
    assert_eq!(category.unwrap().id, "interactive");
}

#[test]
fn test_validate_all_data() {
    let result = slint_showcase_lib::data::validate_all_data();
    assert!(
        result.is_ok(),
        "All demo data should be valid: {:?}",
        result
    );
}

#[test]
fn test_difficulty_default() {
    let demo = CapabilityDemo::new("test", "Test", "Test", "interactive", "test.slint");
    assert_eq!(demo.difficulty, Difficulty::Beginner);
}

#[test]
fn test_difficulty_with_intermediate() {
    let demo = CapabilityDemo::new("test", "Test", "Test", "interactive", "test.slint")
        .with_difficulty(Difficulty::Intermediate);

    assert_eq!(demo.difficulty, Difficulty::Intermediate);
}

#[test]
fn test_difficulty_with_advanced() {
    let demo = CapabilityDemo::new("test", "Test", "Test", "interactive", "test.slint")
        .with_difficulty(Difficulty::Advanced);

    assert_eq!(demo.difficulty, Difficulty::Advanced);
}
