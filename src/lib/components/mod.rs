// Components Module
// Exports all Slint UI components

// Re-export the app.slint component
include_modules!();

pub use app::{App, HomeView, InteractiveDemosView, PerformanceView, ResponsiveView, AccessibilityView, Category, Demo, Metric};
