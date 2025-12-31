//! Integration Tests for Capability Selection
//! Tests verifying demo navigation from categories

use slint_showcase_lib::data::{
    find_category, find_demo, load_categories, load_demos, load_demos_by_category,
};

#[test]
fn test_demos_load_successfully() {
    let demos = load_demos();

    assert!(!demos.is_empty(), "Demos should load successfully");
    assert!(demos.len() >= 5, "Should have at least 5 demos");
}

#[test]
fn test_demos_have_required_fields() {
    let demos = load_demos();

    for demo in demos {
        assert!(!demo.id.is_empty(), "Demo ID should not be empty");
        assert!(!demo.title.is_empty(), "Demo title should not be empty");
        assert!(
            !demo.description.is_empty(),
            "Demo description should not be empty"
        );
        assert!(
            !demo.category.is_empty(),
            "Demo category should not be empty"
        );
        assert!(
            !demo.slint_file.is_empty(),
            "Demo slint_file should not be empty"
        );
    }
}

#[test]
fn test_demos_filter_by_category() {
    let interactive_demos = load_demos_by_category("interactive");

    assert!(
        !interactive_demos.is_empty(),
        "Should have interactive demos"
    );
    for demo in interactive_demos {
        assert_eq!(
            demo.category, "interactive",
            "All filtered demos should belong to interactive category"
        );
    }
}

#[test]
fn test_filter_returns_empty_for_unknown_category() {
    let demos = load_demos_by_category("nonexistent");

    assert!(demos.is_empty(), "Should return empty for unknown category");
}

#[test]
fn test_find_demo_returns_correct_demo() {
    let demo = find_demo("counter");

    assert!(demo.is_some(), "Should find counter demo");
    let demo = demo.unwrap();
    assert_eq!(demo.id, "counter");
    assert_eq!(demo.title, "Counter");
}

#[test]
fn test_find_demo_returns_none_for_unknown() {
    let demo = find_demo("nonexistent-demo");
    assert!(demo.is_none(), "Should return None for unknown demo");
}

#[test]
fn test_all_demo_categories_exist_in_categories() {
    let demos = load_demos();
    let categories = load_categories();
    let category_ids: Vec<&str> = categories.iter().map(|c| c.id.as_str()).collect();

    for demo in demos {
        assert!(
            category_ids.contains(&demo.category.as_str()),
            "Demo '{}' has category '{}' that doesn't exist in categories list",
            demo.id,
            demo.category
        );
    }
}

#[test]
fn test_each_category_has_at_least_one_demo() {
    let categories = load_categories();
    let demos = load_demos();

    for category in categories {
        let category_demos: Vec<_> = demos.iter().filter(|d| d.category == category.id).collect();

        assert!(
            !category_demos.is_empty(),
            "Category '{}' should have at least one demo",
            category.id
        );
    }
}

#[test]
fn test_demo_difficulty_levels_are_valid() {
    let demos = load_demos();

    for demo in demos {
        match &demo.difficulty {
            slint_showcase_lib::data::capability_demo::Difficulty::Beginner => {}
            slint_showcase_lib::data::capability_demo::Difficulty::Intermediate => {}
            slint_showcase_lib::data::capability_demo::Difficulty::Advanced => {}
        }
    }
}

#[test]
fn test_demo_estimated_time_is_reasonable() {
    let demos = load_demos();

    for demo in demos {
        assert!(
            demo.estimated_time >= 1 && demo.estimated_time <= 60,
            "Demo '{}' estimated time should be between 1-60 minutes, got {}",
            demo.id,
            demo.estimated_time
        );
    }
}
