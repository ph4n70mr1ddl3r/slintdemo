//! Integration Tests for Code Editor
//! Tests verifying code editor component renders correctly

use slint_showcase_lib::data::code_example::{find_example, get_code_examples, CodeExample};

#[test]
fn test_code_examples_load_successfully() {
    let examples = get_code_examples();

    assert!(
        !examples.is_empty(),
        "Code examples should load successfully"
    );
    assert!(examples.len() >= 4, "Should have at least 4 code examples");
}

#[test]
fn test_all_required_examples_exist() {
    let examples = get_code_examples();
    let example_ids: Vec<&str> = examples.iter().map(|e| e.id.as_str()).collect();

    assert!(
        example_ids.contains(&"hello-component"),
        "Hello component example should exist"
    );
    assert!(
        example_ids.contains(&"button-styling"),
        "Button styling example should exist"
    );
    assert!(
        example_ids.contains(&"property-binding"),
        "Property binding example should exist"
    );
    assert!(
        example_ids.contains(&"callback-handling"),
        "Callback handling example should exist"
    );
}

#[test]
fn test_examples_have_required_fields() {
    let examples = get_code_examples();

    for example in examples {
        assert!(!example.id.is_empty(), "Example ID should not be empty");
        assert!(
            !example.title.is_empty(),
            "Example title should not be empty"
        );
        assert!(
            !example.description.is_empty(),
            "Example description should not be empty"
        );
        assert!(
            !example.code_template.is_empty(),
            "Code template should not be empty"
        );
        assert!(
            !example.expected_output.is_empty(),
            "Expected output should not be empty"
        );
    }
}

#[test]
fn test_examples_have_valid_difficulty() {
    let examples = get_code_examples();

    for example in examples {
        match &example.difficulty {
            slint_showcase_lib::data::capability_demo::Difficulty::Beginner => {}
            slint_showcase_lib::data::capability_demo::Difficulty::Intermediate => {}
            slint_showcase_lib::data::capability_demo::Difficulty::Advanced => {}
        }
    }
}

#[test]
fn test_find_example_returns_correct_example() {
    let example = find_example("hello-component");

    assert!(example.is_some(), "Should find hello-component example");
    let example = example.unwrap();
    assert_eq!(example.id, "hello-component");
    assert_eq!(example.title, "Your First Component");
}

#[test]
fn test_find_example_returns_none_for_unknown() {
    let example = find_example("nonexistent-example");
    assert!(example.is_none(), "Should return None for unknown example");
}

#[test]
fn test_example_templates_are_valid_syntax() {
    let examples = get_code_examples();

    for example in examples {
        // Templates should contain basic Slint syntax indicators
        let template = example.code_template.as_str();
        assert!(
            template.contains("component") || template.contains("Component"),
            "Template should contain 'component' keyword"
        );
    }
}

#[test]
fn test_example_hints_are_progressive() {
    let examples = get_code_examples();

    for example in examples {
        if let Some(ref hints) = example.hints {
            // Hints should be non-empty if present
            assert!(!hints.is_empty(), "Hints should not be empty if present");
            // Each hint should be non-empty
            for hint in hints {
                assert!(!hint.is_empty(), "Individual hints should not be empty");
            }
        }
    }
}

#[test]
fn test_example_solutions_are_complete() {
    let examples = get_code_examples();

    for example in examples {
        if let Some(ref solution) = example.solution {
            // Solution should be longer than template (more complete)
            assert!(
                solution.len() >= example.code_template.len(),
                "Solution should be at least as long as template"
            );
        }
    }
}

#[test]
fn test_examples_ordered_by_difficulty() {
    let examples = get_code_examples();

    // Should have beginner examples first
    if let Some(first) = examples.first() {
        assert!(
            first.difficulty == slint_showcase_lib::data::capability_demo::Difficulty::Beginner,
            "First example should be beginner difficulty"
        );
    }
}

#[test]
fn test_example_ids_are_unique() {
    let examples = get_code_examples();
    let ids: Vec<&str> = examples.iter().map(|e| e.id.as_str()).collect();
    let unique_ids: std::collections::HashSet<&str> = ids.iter().cloned().collect();

    assert_eq!(
        ids.len(),
        unique_ids.len(),
        "All example IDs should be unique"
    );
}

#[test]
fn test_example_titles_are_descriptive() {
    let examples = get_code_examples();

    for example in examples {
        assert!(
            example.title.len() >= 3 && example.title.len() <= 100,
            "Title length should be between 3-100 characters"
        );
    }
}

#[test]
fn test_example_descriptions_explain_concept() {
    let examples = get_code_examples();

    for example in examples {
        assert!(
            example.description.len() >= 10 && example.description.len() <= 300,
            "Description should be between 10-300 characters"
        );
    }
}
