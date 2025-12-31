// Demo Registry
// Maps demo IDs to their component types and metadata

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Registry entry for a demo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoRegistryEntry {
    pub id: String,
    pub title: String,
    pub component_name: String,
    pub file_path: String,
    pub category: String,
    pub difficulty: String,
    pub estimated_minutes: u32,
}

/// Demo registry - maps demo ID to entry
#[derive(Debug, Clone)]
pub struct DemoRegistry {
    entries: HashMap<String, DemoRegistryEntry>,
    ordered_ids: Vec<String>,
}

impl DemoRegistry {
    /// Create new registry with all demos
    pub fn new() -> Self {
        let entries = HashMap::from([
            // Interactive demos
            (
                "counter".to_string(),
                DemoRegistryEntry {
                    id: "counter".to_string(),
                    title: "Counter Demo".to_string(),
                    component_name: "DemoCounter".to_string(),
                    file_path: "components/demo_counter.slint".to_string(),
                    category: "interactive".to_string(),
                    difficulty: "beginner".to_string(),
                    estimated_minutes: 2,
                },
            ),
            (
                "button-states".to_string(),
                DemoRegistryEntry {
                    id: "button-states".to_string(),
                    title: "Button States".to_string(),
                    component_name: "DemoButton".to_string(),
                    file_path: "components/demo_button.slint".to_string(),
                    category: "interactive".to_string(),
                    difficulty: "beginner".to_string(),
                    estimated_minutes: 3,
                },
            ),
            (
                "text-input".to_string(),
                DemoRegistryEntry {
                    id: "text-input".to_string(),
                    title: "Text Input".to_string(),
                    component_name: "DemoText".to_string(),
                    file_path: "components/demo_text.slint".to_string(),
                    category: "interactive".to_string(),
                    difficulty: "intermediate".to_string(),
                    estimated_minutes: 5,
                },
            ),
            (
                "slider".to_string(),
                DemoRegistryEntry {
                    id: "slider".to_string(),
                    title: "Interactive Slider".to_string(),
                    component_name: "DemoSlider".to_string(),
                    file_path: "components/demo_slider.slint".to_string(),
                    category: "interactive".to_string(),
                    difficulty: "beginner".to_string(),
                    estimated_minutes: 3,
                },
            ),
            (
                "checkbox".to_string(),
                DemoRegistryEntry {
                    id: "checkbox".to_string(),
                    title: "Checkbox Group".to_string(),
                    component_name: "DemoCheckbox".to_string(),
                    file_path: "components/demo_checkbox.slint".to_string(),
                    category: "interactive".to_string(),
                    difficulty: "beginner".to_string(),
                    estimated_minutes: 3,
                },
            ),
        ]);

        let ordered_ids = vec![
            "counter".to_string(),
            "button-states".to_string(),
            "text-input".to_string(),
            "slider".to_string(),
            "checkbox".to_string(),
        ];

        Self {
            entries,
            ordered_ids,
        }
    }

    /// Get a demo by ID
    pub fn get(&self, id: &str) -> Option<&DemoRegistryEntry> {
        self.entries.get(id)
    }

    /// Get all demo IDs in order
    pub fn ids(&self) -> &[String] {
        &self.ordered_ids
    }

    /// Get all demos in order
    pub fn all(&self) -> Vec<&DemoRegistryEntry> {
        self.ordered_ids
            .iter()
            .filter_map(|id| self.entries.get(id))
            .collect()
    }

    /// Get demos by category
    pub fn by_category(&self, category: &str) -> Vec<&DemoRegistryEntry> {
        self.all()
            .into_iter()
            .filter(|d| d.category == category)
            .collect()
    }

    /// Get demo count
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Global demo registry instance
pub static DEMO_REGISTRY: DemoRegistry = DemoRegistry::new();

/// Get the demo registry
pub fn get_demo_registry() -> &'static DemoRegistry {
    &DEMO_REGISTRY
}
