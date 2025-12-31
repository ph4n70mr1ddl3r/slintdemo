//! Integration Tests for Demo Navigation
//! Tests verifying demo list renders and navigation functionality

use slint_showcase_lib::data::{load_categories, load_demos, load_demos_by_category};

#[test]
fn test_demo_list_renders() {
    // Test that demos can be loaded
    let demos = load_demos();
    assert!(!demos.is_empty(), "Demos list should not be empty");
    assert!(
        demos.len() >= 5,
        "Should have at least 5 demos for interactive showcase"
    );
}

#[test]
fn test_demo_has_required_fields() {
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
fn test_demo_categories_exist() {
    let categories = load_categories();
    assert!(
        !categories.is_empty(),
        "Categories list should not be empty"
    );

    // Check that at least interactive category exists
    let interactive = categories.iter().find(|c| c.id == "interactive");
    assert!(interactive.is_some(), "Interactive category should exist");
}

#[test]
fn test_demos_filtered_by_category() {
    let interactive_demos = load_demos_by_category("interactive");
    for demo in interactive_demos {
        assert_eq!(
            demo.category, "interactive",
            "All demos in interactive category should have category='interactive'"
        );
    }
}

#[test]
fn test_demo_id_uniqueness() {
    let demos = load_demos();
    let mut ids: Vec<&String> = demos.iter().map(|d| &d.id).collect();
    ids.sort();
    ids.dedup();

    assert_eq!(ids.len(), demos.len(), "All demo IDs should be unique");
}

#[test]
fn test_demo_difficulty_levels() {
    let demos = load_demos();
    for demo in demos {
        match demo.difficulty {
            slint_showcase_lib::data::Difficulty::Beginner => {}
            slint_showcase_lib::data::Difficulty::Intermediate => {}
            slint_showcase_lib::data::Difficulty::Advanced => {}
        }
    }
}

#[test]
fn test_demo_estimation_time_reasonable() {
    let demos = load_demos();
    for demo in demos {
        assert!(
            demo.estimated_time >= 1 && demo.estimated_time <= 30,
            "Demo estimated time should be between 1-30 minutes, got {} for demo '{}'",
            demo.estimated_time,
            demo.id
        );
    }
}
