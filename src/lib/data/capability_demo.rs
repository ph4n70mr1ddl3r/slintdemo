// CapabilityDemo Entity
// Represents an interactive demonstration of a specific Slint web capability

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityDemo {
    /// Unique identifier (kebab-case)
    pub id: String,
    /// Display name
    pub title: String,
    /// Brief explanation of the demo
    pub description: String,
    /// Category reference
    pub category: String,
    /// Path to .slint source file
    pub slint_file: String,
    /// Configurable demo parameters
    pub parameters: Option<HashMap<String, String>>,
    /// Difficulty level
    pub difficulty: Difficulty,
    /// Estimated time to complete (minutes)
    pub estimated_time: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "beginner")]
    Beginner,
    #[serde(rename = "intermediate")]
    Intermediate,
    #[serde(rename = "advanced")]
    Advanced,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Beginner
    }
}

impl CapabilityDemo {
    /// Create a new demo with required fields
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        category: impl Into<String>,
        slint_file: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            category: category.into(),
            slint_file: slint_file.into(),
            parameters: None,
            difficulty: Difficulty::default(),
            estimated_time: 2,
        }
    }

    /// Validate demo has required non-empty fields
    pub fn validate(&self) -> Result<(), DemoValidationError> {
        if self.id.is_empty() {
            Err(DemoValidationError::EmptyId)
        } else if self.id.len() > 50 {
            Err(DemoValidationError::IdTooLong)
        } else if self.title.is_empty() {
            Err(DemoValidationError::EmptyTitle)
        } else if self.description.len() > 500 {
            Err(DemoValidationError::DescriptionTooLong)
        } else {
            Ok(())
        }
    }

    /// Set difficulty level
    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    /// Set estimated time
    pub fn with_estimated_time(mut self, minutes: u32) -> Self {
        self.estimated_time = minutes;
        self
    }

    /// Set parameters
    pub fn with_parameters(mut self, params: HashMap<String, String>) -> Self {
        self.parameters = Some(params);
        self
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DemoValidationError {
    #[error("Demo ID cannot be empty")]
    EmptyId,
    #[error("Demo ID cannot exceed 50 characters")]
    IdTooLong,
    #[error("Demo title cannot be empty")]
    EmptyTitle,
    #[error("Demo description cannot exceed 500 characters")]
    DescriptionTooLong,
}

/// Get all interactive demos for the showcase
pub fn get_interactive_demos() -> Vec<CapabilityDemo> {
    vec![
        CapabilityDemo::new(
            "counter",
            "Counter",
            "Demonstrates simple state management with increment/decrement",
            "interactive",
            "demos/counter.slint",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_time(2),
        CapabilityDemo::new(
            "button-states",
            "Button States",
            "Shows different button states: normal, pressed, disabled",
            "interactive",
            "demos/button-states.slint",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_time(3),
        CapabilityDemo::new(
            "slider",
            "Interactive Slider",
            "Two-way data binding with slider component",
            "interactive",
            "demos/slider.slint",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_time(3),
        CapabilityDemo::new(
            "text-input",
            "Text Input",
            "Form input with validation and state binding",
            "interactive",
            "demos/text-input.slint",
        )
        .with_difficulty(Difficulty::Intermediate)
        .with_estimated_time(5),
        CapabilityDemo::new(
            "checkbox",
            "Checkbox Group",
            "Multiple selection with state management",
            "interactive",
            "demos/checkbox.slint",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_time(3),
    ]
}

/// Get all demos for the showcase
pub fn get_all_demos() -> Vec<CapabilityDemo> {
    let mut demos = get_interactive_demos();

    // Add performance demos
    demos.push(
        CapabilityDemo::new(
            "animations",
            "Smooth Animations",
            "60fps animations with Slint's animation system",
            "performance",
            "demos/animations.slint",
        )
        .with_difficulty(Difficulty::Intermediate)
        .with_estimated_time(5),
    );

    // Add responsive demos
    demos.push(
        CapabilityDemo::new(
            "layout-grid",
            "Responsive Grid",
            "CSS-like grid layouts that adapt to screen size",
            "responsive",
            "demos/layout-grid.slint",
        )
        .with_difficulty(Difficulty::Intermediate)
        .with_estimated_time(5),
    );

    // Add accessibility demos
    demos.push(
        CapabilityDemo::new(
            "keyboard-nav",
            "Keyboard Navigation",
            "Full keyboard accessibility support",
            "accessibility",
            "demos/keyboard-nav.slint",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_estimated_time(3),
    );

    demos
}
