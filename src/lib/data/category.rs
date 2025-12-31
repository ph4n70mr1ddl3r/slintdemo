// Category Entity
// Represents a logical grouping of related capabilities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier (kebab-case)
    pub id: String,
    /// Display name
    pub name: String,
    /// Description of what this category covers
    pub description: String,
    /// Sort order in the UI
    pub display_order: u32,
    /// Icon identifier (emoji or icon name)
    pub icon: Option<String>,
}

impl Category {
    /// Create a new category with required fields
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        display_order: u32,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            display_order,
            icon: None,
        }
    }

    /// Validate category has required non-empty fields
    pub fn validate(&self) -> Result<(), CategoryValidationError> {
        if self.id.is_empty() {
            Err(CategoryValidationError::EmptyId)
        } else if self.id.len() > 30 {
            Err(CategoryValidationError::IdTooLong)
        } else if self.name.is_empty() {
            Err(CategoryValidationError::EmptyName)
        } else if self.description.is_empty() {
            Err(CategoryValidationError::EmptyDescription)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CategoryValidationError {
    #[error("Category ID cannot be empty")]
    EmptyId,
    #[error("Category ID cannot exceed 30 characters")]
    IdTooLong,
    #[error("Category name cannot be empty")]
    EmptyName,
    #[error("Category description cannot be empty")]
    EmptyDescription,
}

// Predefined categories for the showcase
pub fn get_showcase_categories() -> Vec<Category> {
    vec![
        Category::new(
            "interactive",
            "Interactive Demos",
            "Hands-on demonstrations of Slint's reactive UI capabilities",
            1,
        )
        .with_icon("🎮"),
        Category::new(
            "performance",
            "Performance",
            "Speed and efficiency benchmarks",
            2,
        )
        .with_icon("⚡"),
        Category::new(
            "responsive",
            "Responsive Design",
            "Layouts that adapt to any screen size",
            3,
        )
        .with_icon("📱"),
        Category::new(
            "accessibility",
            "Accessibility",
            "Inclusive design with keyboard and screen reader support",
            4,
        )
        .with_icon("♿"),
    ]
}

impl Category {
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}
