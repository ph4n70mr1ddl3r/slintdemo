//! Integration Tests for Category Navigation
//! Tests verifying category list renders and navigation behavior

use slint_showcase_lib::data::category::Category;
use slint_showcase_lib::data::{find_category, load_categories};

#[test]
fn test_categories_load_successfully() {
    let categories = load_categories();

    assert!(
        !categories.is_empty(),
        "Categories should load successfully"
    );
    assert!(categories.len() >= 4, "Should have at least 4 categories");
}

#[test]
fn test_all_required_categories_exist() {
    let categories = load_categories();
    let category_ids: Vec<&str> = categories.iter().map(|c| c.id.as_str()).collect();

    assert!(
        category_ids.contains(&"interactive"),
        "Interactive category should exist"
    );
    assert!(
        category_ids.contains(&"performance"),
        "Performance category should exist"
    );
    assert!(
        category_ids.contains(&"responsive"),
        "Responsive category should exist"
    );
    assert!(
        category_ids.contains(&"accessibility"),
        "Accessibility category should exist"
    );
}

#[test]
fn test_category_order_is_consistent() {
    let categories = load_categories();

    for window in categories.windows(2) {
        let first = &window[0];
        let second = &window[1];
        assert!(
            first.display_order <= second.display_order,
            "Categories should be sorted by display_order: {} ({}) should be <= {} ({})",
            first.name,
            first.display_order,
            second.name,
            second.display_order
        );
    }
}

#[test]
fn test_category_ids_are_unique() {
    let categories = load_categories();
    let ids: Vec<&str> = categories.iter().map(|c| c.id.as_str()).collect();
    let unique_ids: std::collections::HashSet<&str> = ids.iter().cloned().collect();

    assert_eq!(
        ids.len(),
        unique_ids.len(),
        "All category IDs should be unique"
    );
}

#[test]
fn test_find_category_returns_correct_category() {
    let category = find_category("interactive");

    assert!(category.is_some(), "Should find interactive category");
    let category = category.unwrap();
    assert_eq!(category.id, "interactive");
    assert_eq!(category.name, "Interactive Demos");
}

#[test]
fn test_find_category_returns_none_for_unknown() {
    let category = find_category("nonexistent-category");
    assert!(
        category.is_none(),
        "Should return None for unknown category"
    );
}

#[test]
fn test_category_has_required_fields() {
    let categories = load_categories();

    for category in categories {
        assert!(!category.id.is_empty(), "Category ID should not be empty");
        assert!(
            !category.name.is_empty(),
            "Category name should not be empty"
        );
        assert!(
            !category.description.is_empty(),
            "Category description should not be empty"
        );
        assert!(
            category.display_order >= 0,
            "Display order should be non-negative"
        );
    }
}

#[test]
fn test_category_icon_is_valid() {
    let categories = load_categories();

    for category in categories {
        if let Some(icon) = &category.icon {
            assert!(!icon.is_empty(), "Icon should not be empty if present");
        }
    }
}
