// Code Example Entity
// Represents an editable code snippet for hands-on learning

use super::capability_demo::Difficulty;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeExample {
    pub id: String,
    pub title: String,
    pub description: String,
    pub code_template: String,
    pub expected_output: String,
    pub difficulty: Difficulty,
    pub hints: Option<Vec<String>>,
    pub solution: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ExampleValidationError {
    #[error("Example ID cannot be empty")]
    EmptyId,
    #[error("Example ID cannot exceed 50 characters")]
    IdTooLong,
    #[error("Example title cannot be empty")]
    EmptyTitle,
    #[error("Example description cannot be empty")]
    EmptyDescription,
    #[error("Code template cannot be empty")]
    EmptyTemplate,
    #[error("Expected output cannot be empty")]
    EmptyOutput,
}

impl CodeExample {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        code_template: impl Into<String>,
        expected_output: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            code_template: code_template.into(),
            expected_output: expected_output.into(),
            difficulty: Difficulty::Beginner,
            hints: None,
            solution: None,
        }
    }

    pub fn validate(&self) -> Result<(), ExampleValidationError> {
        if self.id.is_empty() {
            Err(ExampleValidationError::EmptyId)
        } else if self.id.len() > 50 {
            Err(ExampleValidationError::IdTooLong)
        } else if self.title.is_empty() {
            Err(ExampleValidationError::EmptyTitle)
        } else if self.description.is_empty() {
            Err(ExampleValidationError::EmptyDescription)
        } else if self.code_template.is_empty() {
            Err(ExampleValidationError::EmptyTemplate)
        } else if self.expected_output.is_empty() {
            Err(ExampleValidationError::EmptyOutput)
        } else {
            Ok(())
        }
    }

    pub fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    pub fn with_hints(mut self, hints: Vec<String>) -> Self {
        self.hints = Some(hints);
        self
    }

    pub fn with_solution(mut self, solution: impl Into<String>) -> Self {
        self.solution = Some(solution.into());
        self
    }
}

fn get_default_examples() -> Vec<CodeExample> {
    vec![
        CodeExample::new(
            "hello-component",
            "Your First Component",
            "Learn to create a basic Slint component with properties and callbacks",
            r#"component MyButton {
    in property <string> text;
    in property <color> primary-color;
    
    callback clicked();
    
    Rectangle {
        background: primary-color;
        width: 120px;
        height: 40px;
        
        Text {
            text: text;
            color: white;
        }
    }
}

export component Main {
    MyButton {
        text: "Click Me!";
        primary-color: #2563eb;
        clicked => {
            debug("Button clicked!");
        }
    }
}"#,
            "A clickable button component with blue background and 'Click Me!' text",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_hints(vec![
            "Components start with 'component' keyword".to_string(),
            "Use 'in property' for input properties".to_string(),
            "The exported component is your entry point".to_string(),
        ])
        .with_solution(
            r#"component MyButton {
    in property <string> text;
    in property <color> primary-color;
    
    callback clicked();
    
    Rectangle {
        background: primary-color;
        width: 120px;
        height: 40px;
        border-radius: 8px;
        
        Text {
            text: text;
            color: white;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

export component Main {
    MyButton {
        text: "Click Me!";
        primary-color: #2563eb;
        clicked => {
            debug("Button clicked!");
        }
    }
}"#
            .to_string(),
        ),
        CodeExample::new(
            "button-styling",
            "Button Styling",
            "Explore different ways to style buttons with colors, borders, and states",
            r#"component StyledButton {
    in property <string> text;
    in property <color> background-color;
    in property <color> text-color;
    in property <bool> disabled;
    
    callback clicked();
    
    Rectangle {
        background: disabled ? #cccccc : background-color;
        width: 100px;
        height: 36px;
        border-radius: 6px;
        opacity: disabled ? 0.7 : 1.0;
        
        Text {
            text: text;
            color: disabled ? #888888 : text-color;
        }
    }
}"#,
            "A styled button with configurable colors and disabled state",
        )
        .with_difficulty(Difficulty::Beginner)
        .with_hints(vec![
            "Use conditional expressions for states".to_string(),
            "Opacity can create disabled effects".to_string(),
        ])
        .with_solution(
            r#"component StyledButton {
    in property <string> text;
    in property <color> background-color;
    in property <color> text-color;
    in property <bool> disabled;
    
    callback clicked();
    
    Rectangle {
        background: disabled ? #cccccc : background-color;
        width: 100px;
        height: 36px;
        border-radius: 6px;
        opacity: disabled ? 0.7 : 1.0;
        border-width: disabled ? 0 : 2px;
        border-color: disabled ? transparent : background-color.darker(20%);
        
        Text {
            text: text;
            color: disabled ? #888888 : text-color;
            font-size: 14px;
            font-weight: 600;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}"#
            .to_string(),
        ),
        CodeExample::new(
            "property-binding",
            "Property Binding",
            "Master bidirectional and conditional property bindings",
            r#"component Counter {
    in-out property <int> count;
    
    callback increment();
    callback decrement();
    
    HorizontalBox {
        Button {
            text: "-";
            clicked => { decrement(); }
        }
        
        Text {
            text: count;
            font-size: 24px;
            font-weight: 700;
        }
        
        Button {
            text: "+";
            clicked => { increment(); }
        }
    }
}"#,
            "A counter with increment/decrement buttons and display",
        )
        .with_difficulty(Difficulty::Intermediate)
        .with_hints(vec![
            "'in-out property' allows both reading and writing".to_string(),
            "Callbacks can modify the property".to_string(),
        ])
        .with_solution(
            r#"component Counter {
    in-out property <int> count;
    in property <int> minimum: 0;
    in property <int> maximum: 100;
    
    callback increment();
    callback decrement();
    
    function handle-increment() {
        if count < maximum {
            count += 1;
            increment();
        }
    }
    
    function handle-decrement() {
        if count > minimum {
            count -= 1;
            decrement();
        }
    }
    
    HorizontalBox {
        spacing: 8px;
        alignment: center;
        
        Button {
            text: "-";
            clicked => { handle-decrement(); }
        }
        
        Rectangle {
            width: 60px;
            
            Text {
                text: count;
                font-size: 24px;
                font-weight: 700;
                horizontal-alignment: center;
                vertical-alignment: center;
                color: count == maximum ? #2563eb : (count == minimum ? #dc2626 : #000000);
            }
        }
        
        Button {
            text: "+";
            clicked => { handle-increment(); }
        }
    }
}"#
            .to_string(),
        ),
        CodeExample::new(
            "callback-handling",
            "Callback Handling",
            "Implement event handlers and callback chains for complex interactions",
            r#"component ToggleSwitch {
    in-out property <bool> checked;
    
    callback toggled(bool);
    
    Rectangle {
        width: 50px;
        height: 28px;
        background: checked ? #22c55e : #6b7280;
        border-radius: 14px;
        
        Rectangle {
            x: checked ? 24px : 2px;
            width: 24px;
            height: 24px;
            background: white;
            border-radius: 12px;
        }
    }
}"#,
            "A toggle switch with animated state change",
        )
        .with_difficulty(Difficulty::Intermediate)
        .with_hints(vec![
            "Callbacks can pass parameters".to_string(),
            "Animation can be added with 'animate' keyword".to_string(),
        ])
        .with_solution(
            r#"component ToggleSwitch {
    in-out property <bool> checked;
    in property <color> on-color: #22c55e;
    in property <color> off-color: #6b7280;
    
    callback toggled(bool);
    
    Rectangle {
        width: 50px;
        height: 28px;
        background: checked ? on-color : off-color;
        border-radius: 14px;
        animate background {
            duration: 200ms;
            easing: ease-in-out;
        }
        
        Rectangle {
            x: checked ? 24px : 2px;
            width: 24px;
            height: 24px;
            background: white;
            border-radius: 12px;
            animate x {
                duration: 200ms;
                easing: ease-in-out;
            }
        }
        
        TouchArea {
            clicked => {
                checked = !checked;
                toggled(checked);
            }
        }
    }
}"#
            .to_string(),
        ),
    ]
}

/// Get all code examples
pub fn get_code_examples() -> Vec<CodeExample> {
    get_default_examples()
}

/// Find a code example by ID
pub fn find_example(example_id: &str) -> Option<CodeExample> {
    get_default_examples()
        .into_iter()
        .find(|e| e.id == example_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_new() {
        let example = CodeExample::new("test", "Test", "Description", "code", "output");

        assert_eq!(example.id, "test");
        assert_eq!(example.difficulty, Difficulty::Beginner);
    }

    #[test]
    fn test_example_with_hints() {
        let example = CodeExample::new("test", "Test", "Desc", "code", "output")
            .with_hints(vec!["hint1".to_string()]);

        assert!(example.hints.is_some());
        assert_eq!(example.hints.unwrap().len(), 1);
    }
}
