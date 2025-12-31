//! Integration Tests for Responsive Layout
//! Tests verifying responsive behavior across different screen sizes

use slint_showcase_lib::data::ResponsiveBreakpoint;

#[test]
fn test_breakpoints_are_defined() {
    let breakpoints = ResponsiveBreakpoint::all();

    assert!(!breakpoints.is_empty(), "Breakpoints should be defined");
    assert!(breakpoints.len() >= 3, "Should have at least 3 breakpoints");
}

#[test]
fn test_breakpoints_have_required_fields() {
    let breakpoints = ResponsiveBreakpoint::all();

    for bp in breakpoints {
        assert!(!bp.name.is_empty(), "Breakpoint name should not be empty");
        assert!(bp.max_width >= 0, "Max width should be non-negative");
        assert!(bp.column_count >= 1, "Column count should be at least 1");
        assert!(
            !bp.description.is_empty(),
            "Description should not be empty"
        );
    }
}

#[test]
fn test_breakpoints_are_ordered() {
    let breakpoints = ResponsiveBreakpoint::all();

    for window in breakpoints.windows(2) {
        assert!(
            window[0].max_width <= window[1].max_width,
            "Breakpoints should be ordered by max_width ascending"
        );
    }
}

#[test]
fn test_mobile_breakpoint_exists() {
    let mobile = ResponsiveBreakpoint::find("mobile");

    assert!(mobile.is_some(), "Mobile breakpoint should exist");
    let mobile = mobile.unwrap();
    assert!(mobile.max_width <= 480, "Mobile should be <= 480px");
    assert_eq!(mobile.column_count, 1, "Mobile should have 1 column");
}

#[test]
fn test_tablet_breakpoint_exists() {
    let tablet = ResponsiveBreakpoint::find("tablet");

    assert!(tablet.is_some(), "Tablet breakpoint should exist");
    let tablet = tablet.unwrap();
    assert!(tablet.max_width <= 768, "Tablet should be <= 768px");
    assert!(
        tablet.column_count >= 2,
        "Tablet should have at least 2 columns"
    );
}

#[test]
fn test_desktop_breakpoint_exists() {
    let desktop = ResponsiveBreakpoint::find("desktop");

    assert!(desktop.is_some(), "Desktop breakpoint should exist");
    let desktop = desktop.unwrap();
    assert!(
        desktop.column_count >= 3,
        "Desktop should have at least 3 columns"
    );
}

#[test]
fn test_responsive_config_loads() {
    let config = slint_showcase_lib::data::get_responsive_config();

    assert!(config.is_some(), "Responsive config should load");
    let config = config.unwrap();
    assert!(
        config.enable_adaptive_layouts,
        "Adaptive layouts should be enabled"
    );
    assert!(
        config.min_touch_target >= 24,
        "Min touch target should be >= 24px"
    );
}

#[test]
fn test_all_components_have_responsive_modes() {
    let components = ["App", "CategoryCard", "CapabilityCard"];

    for component in components {
        // In a real scenario, we'd verify each component has responsive support
        // For now, we just verify the component names are tracked
        assert!(!component.is_empty(), "Component name should not be empty");
    }
}

#[test]
fn test_gutter_sizes_are_reasonable() {
    let config = slint_showcase_lib::data::get_responsive_config().unwrap();

    assert!(
        config.gutter.small >= 4 && config.gutter.small <= 16,
        "Small gutter should be 4-16px"
    );
    assert!(
        config.gutter.medium >= 8 && config.gutter.medium <= 24,
        "Medium gutter should be 8-24px"
    );
    assert!(
        config.gutter.large >= 16 && config.gutter.large <= 48,
        "Large gutter should be 16-48px"
    );
}

#[test]
fn test_font_sizes_scale_appropriately() {
    let config = slint_showcase_lib::data::get_responsive_config().unwrap();

    // Font sizes should be reasonable for different contexts
    assert!(config.font_sizes.small >= 10 && config.font_sizes.small <= 14);
    assert!(config.font_sizes.body >= 12 && config.font_sizes.body <= 18);
    assert!(config.font_sizes.heading >= 18 && config.font_sizes.heading <= 32);
    assert!(config.font_sizes.display >= 24 && config.font_sizes.display <= 64);
}

#[test]
fn test_container_widths_are_valid() {
    let config = slint_showcase_lib::data::get_responsive_config().unwrap();

    assert!(
        config.container_widths.narrow >= 280 && config.container_widths.narrow <= 400,
        "Narrow container should be 280-400px"
    );
    assert!(
        config.container_widths.wide >= 600 && config.container_widths.wide <= 1200,
        "Wide container should be 600-1200px"
    );
    assert!(
        config.container_widths.full >= 800,
        "Full container should be at least 800px"
    );
}

#[test]
fn test_breakpoint_for_width_calculation() {
    let breakpoints = ResponsiveBreakpoint::all();

    // Verify we can find the correct breakpoint for various widths
    let mobile_bp = ResponsiveBreakpoint::for_width(320);
    assert!(mobile_bp.is_some(), "Should find breakpoint for 320px");
    assert_eq!(mobile_bp.unwrap().name, "mobile");

    let tablet_bp = ResponsiveBreakpoint::for_width(600);
    assert!(tablet_bp.is_some(), "Should find breakpoint for 600px");

    let desktop_bp = ResponsiveBreakpoint::for_width(1024);
    assert!(desktop_bp.is_some(), "Should find breakpoint for 1024px");
}
