// Keyboard Navigation
// Defines keyboard shortcuts and navigation patterns for accessibility

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyboardShortcut {
    pub action: String,
    pub key: String,
    pub modifiers: Vec<String>,
    pub description: String,
}

impl KeyboardShortcut {
    pub fn new(
        action: impl Into<String>,
        key: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            action: action.into(),
            key: key.into(),
            modifiers: Vec::new(),
            description: description.into(),
        }
    }

    pub fn with_modifiers(mut self, modifiers: Vec<String>) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub fn all() -> Vec<Self> {
        vec![
            KeyboardShortcut::new("next", "Tab", "Move to next focusable element")
                .with_modifiers(vec!["shift".to_string()]),
            KeyboardShortcut::new("previous", "Tab", "Move to previous focusable element"),
            KeyboardShortcut::new("home", "Home", "Go to first item"),
            KeyboardShortcut::new("end", "End", "Go to last item"),
            KeyboardShortcut::new("escape", "Escape", "Close modal or cancel action"),
            KeyboardShortcut::new("enter", "Enter", "Activate button or link"),
            KeyboardShortcut::new("space", "Space", "Toggle checkbox or activate button"),
            KeyboardShortcut::new("arrow-up", "ArrowUp", "Move selection up"),
            KeyboardShortcut::new("arrow-down", "ArrowDown", "Move selection down"),
            KeyboardShortcut::new("arrow-left", "ArrowLeft", "Move selection left"),
            KeyboardShortcut::new("arrow-right", "ArrowRight", "Move selection right"),
        ]
    }

    pub fn find(action: &str) -> Option<Self> {
        Self::all().into_iter().find(|s| s.action == action)
    }

    pub fn to_string(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            format!("{}+{}", self.modifiers.join("+"), self.key)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortcut_all_returns_multiple() {
        let shortcuts = KeyboardShortcut::all();
        assert!(shortcuts.len() >= 5);
    }

    #[test]
    fn test_shortcut_find() {
        let escape = KeyboardShortcut::find("escape");
        assert!(escape.is_some());
        assert_eq!(escape.unwrap().key, "Escape");
    }

    #[test]
    fn test_shortcut_to_string_no_modifiers() {
        let shortcut = KeyboardShortcut::new("test", "A", "Test");
        assert_eq!(shortcut.to_string(), "A");
    }

    #[test]
    fn test_shortcut_to_string_with_modifiers() {
        let shortcut = KeyboardShortcut::new("test", "S", "Test")
            .with_modifiers(vec!["ctrl".to_string(), "shift".to_string()]);
        assert_eq!(shortcut.to_string(), "ctrl+shift+S");
    }
}
