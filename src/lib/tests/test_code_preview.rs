//! Integration Tests for Code Preview
//! Tests verifying code preview updates and live preview behavior

use slint_showcase_lib::data::code_example::{find_example, get_code_examples, CodeExample};

#[test]
fn test_preview_panel_shows_selected_example() {
    let examples = get_code_examples();

    // Simulate selecting an example and showing preview
    let example = find_example("hello-component");
    assert!(example.is_some(), "Should find hello-component example");

    let example = example.unwrap();
    // Preview should show the expected output
    assert!(
        !example.expected_output.is_empty(),
        "Preview should have expected output"
    );
}

#[test]
fn test_example_switching_updates_preview() {
    let hello = find_example("hello-component").unwrap();
    let button = find_example("button-styling").unwrap();

    // Different examples should have different outputs
    assert_ne!(
        hello.expected_output, button.expected_output,
        "Switching examples should update preview"
    );
}

#[test]
fn test_all_examples_have_expected_outputs() {
    let examples = get_code_examples();

    for example in examples {
        assert!(
            !example.expected_output.is_empty(),
            "Example '{}' should have expected output",
            example.id
        );
    }
}

#[test]
fn test_example_templates_are_editable() {
    let examples = get_code_examples();

    for example in examples {
        // Templates should be reasonable length for editing
        assert!(
            example.code_template.len() >= 50,
            "Template should be substantial for editing"
        );
        assert!(
            example.code_template.len() <= 2000,
            "Template should not be excessively long"
        );
    }
}

#[test]
fn test_beginner_examples_have_hints() {
    let examples = get_code_examples();

    // Find beginner examples
    let beginner_examples: Vec<_> = examples
        .iter()
        .filter(|e| e.difficulty == slint_showcase_lib::data::capability_demo::Difficulty::Beginner)
        .collect();

    // At least beginner examples should have hints
    assert!(
        !beginner_examples.is_empty(),
        "Should have beginner examples"
    );

    for example in beginner_examples {
        assert!(
            example.hints.is_some() && !example.hints.as_ref().unwrap().is_empty(),
            "Beginner example '{}' should have hints",
            example.id
        );
    }
}

#[test]
fn test_intermediate_and_advanced_have_fewer_hints() {
    let examples = get_code_examples();

    let advanced_examples: Vec<_> = examples
        .iter()
        .filter(|e| e.difficulty == slint_showcase_lib::data::capability_demo::Difficulty::Advanced)
        .collect();

    for example in advanced_examples {
        // Advanced examples may or may not have hints
        // If they do, they should be fewer than beginner examples
        if let Some(ref hints) = example.hints {
            assert!(
                hints.len() <= 3,
                "Advanced example should have at most 3 hints"
            );
        }
    }
}

#[test]
fn test_example_previews_are_different_from_templates() {
    let examples = get_code_examples();

    for example in examples {
        // Expected output is typically different from code template
        // (one is description, one is code)
        assert_ne!(
            example.expected_output, example.code_template,
            "Preview output should differ from code template"
        );
    }
}

#[test]
fn test_examples_can_be_filtered_by_difficulty() {
    let examples = get_code_examples();

    let beginner = examples
        .iter()
        .filter(|e| e.difficulty == slint_showcase_lib::data::capability_demo::Difficulty::Beginner)
        .collect::<Vec<_>>();

    let intermediate = examples
        .iter()
        .filter(|e| {
            e.difficulty == slint_showcase_lib::data::capability_demo::Difficulty::Intermediate
        })
        .collect::<Vec<_>>();

    // Should have at least beginner and intermediate examples
    assert!(!beginner.is_empty(), "Should have beginner examples");
    assert!(
        !intermediate.is_empty(),
        "Should have intermediate examples"
    );

    // All examples should be categorized in beginner or intermediate
    assert_eq!(
        beginner.len() + intermediate.len(),
        examples.len(),
        "All examples should be categorized by difficulty"
    );
}

#[test]
fn test_code_templates_contain_slint_syntax() {
    let examples = get_code_examples();

    for example in examples {
        let template = example.code_template.as_str();
        // Should contain Slint-specific syntax
        assert!(
            template.contains("component")
                || template.contains("in property")
                || template.contains("out property")
                || template.contains("callback"),
            "Template should contain Slint syntax"
        );
    }
}

#[test]
fn test_solution_when_available_is_complete() {
    let examples = get_code_examples();

    for example in examples {
        if let Some(ref solution) = example.solution {
            // Solution should be valid Slint code
            assert!(
                solution.contains("component") || solution.contains("Component"),
                "Solution should contain component definition"
            );
        }
    }
}

#[test]
fn test_example_list_can_be_iterated() {
    let examples = get_code_examples();
    let count = examples.iter().count();

    assert!(count > 0, "Should be able to iterate over examples");
    assert_eq!(count, examples.len(), "Count should match length");
}
