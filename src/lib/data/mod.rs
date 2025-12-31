// Data Module
// Exports all data types and provides demo data loading functionality

pub mod benchmarks;
pub mod capability_demo;
pub mod category;
pub mod code_example;
pub mod keyboard;
pub mod performance_metric;
pub mod responsive;

pub use benchmarks::{
    get_comparison_data, get_performance_metrics, get_slint_advantages, ComparisonData,
    ComparisonResult, SlintAdvantage,
};
pub use capability_demo::{
    get_all_demos, get_interactive_demos, CapabilityDemo, DemoValidationError, Difficulty,
};
pub use category::{Category, CategoryValidationError};
pub use code_example::{find_example, get_code_examples, CodeExample, ExampleValidationError};
pub use keyboard::KeyboardShortcut;
pub use performance_metric::{MetricType, MetricValidationError, PerformanceMetric};
pub use responsive::{get_responsive_config, ResponsiveBreakpoint, ResponsiveConfig};

/// Load all demo categories
pub fn load_categories() -> Vec<Category> {
    category::get_showcase_categories()
}

/// Load all demos
pub fn load_demos() -> Vec<CapabilityDemo> {
    capability_demo::get_all_demos()
}

/// Load demos by category
pub fn load_demos_by_category(category_id: &str) -> Vec<CapabilityDemo> {
    capability_demo::get_all_demos()
        .into_iter()
        .filter(|demo| demo.category == category_id)
        .collect()
}

/// Find a demo by ID
pub fn find_demo(demo_id: &str) -> Option<CapabilityDemo> {
    capability_demo::get_all_demos()
        .into_iter()
        .find(|demo| demo.id == demo_id)
}

/// Find a category by ID
pub fn find_category(category_id: &str) -> Option<Category> {
    category::get_showcase_categories()
        .into_iter()
        .find(|cat| cat.id == category_id)
}

/// Validate all loaded data
pub fn validate_all_data() -> Result<(), DataValidationError> {
    for category in load_categories() {
        category
            .validate()
            .map_err(|e| DataValidationError::CategoryError(category.id.clone(), e))?;
    }

    for demo in load_demos() {
        demo.validate()
            .map_err(|e| DataValidationError::DemoError(demo.id.clone(), e))?;
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum DataValidationError {
    #[error("Category {0} validation failed: {1}")]
    CategoryError(String, #[source] category::CategoryValidationError),
    #[error("Demo {0} validation failed: {1}")]
    DemoError(String, #[source] capability_demo::DemoValidationError),
}
