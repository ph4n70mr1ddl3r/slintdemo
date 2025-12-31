//! Unit Tests for Responsive Container Components
//! Tests verifying responsive container types and behavior

use slint_showcase_lib::data::responsive::{
    get_responsive_config, GutterSizes, ResponsiveBreakpoint, ResponsiveConfig,
};

#[test]
fn test_responsive_breakpoint_new() {
    let bp = ResponsiveBreakpoint::new("test", 500, 2, "Test breakpoint");

    assert_eq!(bp.name, "test");
    assert_eq!(bp.max_width, 500);
    assert_eq!(bp.column_count, 2);
    assert_eq!(bp.description, "Test breakpoint");
}

#[test]
fn test_responsive_breakpoint_all_contains_expected() {
    let breakpoints = ResponsiveBreakpoint::all();
    let names: Vec<&str> = breakpoints.iter().map(|b| b.name.as_str()).collect();

    assert!(names.contains(&"mobile"));
    assert!(names.contains(&"tablet"));
    assert!(names.contains(&"desktop"));
}

#[test]
fn test_responsive_breakpoint_find() {
    let mobile = ResponsiveBreakpoint::find("mobile");

    assert!(mobile.is_some());
    let mobile = mobile.unwrap();
    assert_eq!(mobile.max_width, 480);
    assert_eq!(mobile.column_count, 1);
}

#[test]
fn test_responsive_breakpoint_for_width_mobile() {
    let bp = ResponsiveBreakpoint::for_width(320);

    assert!(bp.is_some());
    assert_eq!(bp.unwrap().name, "mobile");
}

#[test]
fn test_responsive_breakpoint_for_width_tablet() {
    let bp = ResponsiveBreakpoint::for_width(600);

    assert!(bp.is_some());
    assert_eq!(bp.unwrap().name, "tablet");
}

#[test]
fn test_responsive_breakpoint_for_width_desktop() {
    let bp = ResponsiveBreakpoint::for_width(1024);

    assert!(bp.is_some());
    assert_eq!(bp.unwrap().name, "desktop");
}

#[test]
fn test_responsive_config_new() {
    let config = ResponsiveConfig {
        breakpoints: ResponsiveBreakpoint::all(),
        gutter: GutterSizes {
            small: 8,
            medium: 16,
            large: 24,
        },
        font_sizes: Default::default(),
        container_widths: Default::default(),
        min_touch_target: 44,
        enable_adaptive_layouts: true,
    };

    assert!(config.enable_adaptive_layouts);
    assert_eq!(config.min_touch_target, 44);
}

#[test]
fn test_gutter_sizes_reasonable() {
    let gutter = GutterSizes {
        small: 8,
        medium: 16,
        large: 24,
    };

    assert!(gutter.small < gutter.medium);
    assert!(gutter.medium < gutter.large);
}

#[test]
fn test_breakpoints_ordered_by_max_width() {
    let breakpoints = ResponsiveBreakpoint::all();

    for i in 1..breakpoints.len() {
        assert!(
            breakpoints[i - 1].max_width <= breakpoints[i].max_width,
            "Breakpoints should be ordered by max_width ascending"
        );
    }
}

#[test]
fn test_column_count_increases_with_size() {
    let breakpoints = ResponsiveBreakpoint::all();

    for i in 1..breakpoints.len() {
        assert!(
            breakpoints[i - 1].column_count <= breakpoints[i].column_count,
            "Column count should increase or stay same for larger breakpoints"
        );
    }
}

#[test]
fn test_min_touch_target_meets_accessibility() {
    // WCAG 2.1 recommends 44x44px minimum touch targets
    let config = slint_showcase_lib::data::get_responsive_config().unwrap();

    assert!(
        config.min_touch_target >= 44,
        "Min touch target should be at least 44px for accessibility"
    );
}

#[test]
fn test_breakpoint_names_are_kebab_case() {
    let breakpoints = ResponsiveBreakpoint::all();

    for bp in breakpoints {
        assert!(
            !bp.name.contains(' '),
            "Breakpoint name '{}' should be kebab-case",
            bp.name
        );
        assert!(!bp.name.is_empty(), "Breakpoint name should not be empty");
    }
}

#[test]
fn test_responsive_config_has_all_breakpoints() {
    let config = slint_showcase_lib::data::get_responsive_config().unwrap();

    assert!(
        config.breakpoints.len() >= 3,
        "Should have at least 3 breakpoints"
    );

    let has_mobile = config.breakpoints.iter().any(|b| b.name == "mobile");
    let has_tablet = config.breakpoints.iter().any(|b| b.name == "tablet");
    let has_desktop = config.breakpoints.iter().any(|b| b.name == "desktop");

    assert!(has_mobile, "Should have mobile breakpoint");
    assert!(has_tablet, "Should have tablet breakpoint");
    assert!(has_desktop, "Should have desktop breakpoint");
}
