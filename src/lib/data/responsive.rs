// Responsive Configuration
// Defines responsive behavior and breakpoints for cross-device compatibility

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponsiveBreakpoint {
    pub name: String,
    pub max_width: i32,
    pub column_count: i32,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GutterSizes {
    pub small: i32,
    pub medium: i32,
    pub large: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FontSizes {
    pub small: i32,
    pub body: i32,
    pub heading: i32,
    pub display: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerWidths {
    pub narrow: i32,
    pub wide: i32,
    pub full: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponsiveConfig {
    pub breakpoints: Vec<ResponsiveBreakpoint>,
    pub gutter: GutterSizes,
    pub font_sizes: FontSizes,
    pub container_widths: ContainerWidths,
    pub min_touch_target: i32,
    pub enable_adaptive_layouts: bool,
}

impl ResponsiveBreakpoint {
    pub fn new(
        name: impl Into<String>,
        max_width: i32,
        column_count: i32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            max_width,
            column_count,
            description: description.into(),
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            ResponsiveBreakpoint::new("mobile", 480, 1, "Mobile devices"),
            ResponsiveBreakpoint::new("tablet", 768, 2, "Tablet devices"),
            ResponsiveBreakpoint::new("desktop", 1200, 3, "Desktop displays"),
            ResponsiveBreakpoint::new("wide", 1600, 4, "Wide displays"),
        ]
    }

    pub fn find(name: &str) -> Option<Self> {
        Self::all().into_iter().find(|bp| bp.name == name)
    }

    pub fn for_width(width: i32) -> Option<Self> {
        Self::all().into_iter().find(|bp| width <= bp.max_width)
    }
}

fn get_default_config() -> ResponsiveConfig {
    ResponsiveConfig {
        breakpoints: ResponsiveBreakpoint::all(),
        gutter: GutterSizes {
            small: 8,
            medium: 16,
            large: 24,
        },
        font_sizes: FontSizes {
            small: 12,
            body: 14,
            heading: 20,
            display: 32,
        },
        container_widths: ContainerWidths {
            narrow: 320,
            wide: 800,
            full: 1200,
        },
        min_touch_target: 44,
        enable_adaptive_layouts: true,
    }
}

/// Get responsive configuration
pub fn get_responsive_config() -> Option<ResponsiveConfig> {
    Some(get_default_config())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_all_returns_multiple() {
        let breakpoints = ResponsiveBreakpoint::all();
        assert!(breakpoints.len() >= 3);
    }

    #[test]
    fn test_breakpoint_find() {
        let mobile = ResponsiveBreakpoint::find("mobile");
        assert!(mobile.is_some());
        assert_eq!(mobile.unwrap().name, "mobile");
    }

    #[test]
    fn test_breakpoint_for_width() {
        let bp = ResponsiveBreakpoint::for_width(400);
        assert!(bp.is_some());
        assert_eq!(bp.unwrap().name, "mobile");
    }
}
