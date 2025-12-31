//! Unit Tests for Code Example Structures
//! Tests verifying code example types and validation

use slint_showcase_lib::data::capability_demo::Difficulty;
use slint_showcase_lib::data::code_example::{CodeExample, ExampleValidationError};

#[test]
fn test_code_example_new() {
    let example = CodeExample::new(
        "test-example",
        "Test Example",
        "A test description",
        "component Test { }",
        "A test component",
    );

    assert_eq!(example.id, "test-example");
    assert_eq!(example.title, "Test Example");
    assert_eq!(example.description, "A test description");
    assert_eq!(example.code_template, "component Test { }");
    assert_eq!(example.expected_output, "A test component");
    assert_eq!(example.difficulty, Difficulty::Beginner);
    assert!(example.hints.is_none());
    assert!(example.solution.is_none());
}

#[test]
fn test_code_example_with_difficulty() {
    let example = CodeExample::new("test", "Test", "Desc", "code", "output")
        .with_difficulty(Difficulty::Advanced);

    assert_eq!(example.difficulty, Difficulty::Advanced);
}

#[test]
fn test_code_example_with_hints() {
    let example = CodeExample::new("test", "Test", "Desc", "code", "output")
        .with_hints(vec!["Hint 1".to_string(), "Hint 2".to_string()]);

    assert!(example.hints.is_some());
    let hints = example.hints.unwrap();
    assert_eq!(hints.len(), 2);
    assert_eq!(hints[0], "Hint 1");
}

#[test]
fn test_code_example_with_solution() {
    let example =
        CodeExample::new("test", "Test", "Desc", "code", "output").with_solution("full solution");

    assert!(example.solution.is_some());
    assert_eq!(example.solution.unwrap(), "full solution");
}

#[test]
fn test_code_example_validation_empty_id() {
    let example = CodeExample::new("", "Title", "Description", "code", "output");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::EmptyId)
    ));
}

#[test]
fn test_code_example_validation_id_too_long() {
    let long_id = "x".repeat(51);
    let example = CodeExample::new(long_id, "Title", "Description", "code", "output");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::IdTooLong)
    ));
}

#[test]
fn test_code_example_validation_empty_title() {
    let example = CodeExample::new("test-id", "", "Description", "code", "output");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::EmptyTitle)
    ));
}

#[test]
fn test_code_example_validation_empty_description() {
    let example = CodeExample::new("test-id", "Title", "", "code", "output");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::EmptyDescription)
    ));
}

#[test]
fn test_code_example_validation_empty_template() {
    let example = CodeExample::new("test-id", "Title", "Description", "", "output");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::EmptyTemplate)
    ));
}

#[test]
fn test_code_example_validation_empty_output() {
    let example = CodeExample::new("test-id", "Title", "Description", "code", "");

    assert!(matches!(
        example.validate(),
        Err(ExampleValidationError::EmptyOutput)
    ));
}

#[test]
fn test_code_example_validation_valid() {
    let example = CodeExample::new(
        "valid-id",
        "Valid Title",
        "Valid Description",
        "component Test { }",
        "Valid Output",
    );

    assert!(example.validate().is_ok());
}

#[test]
fn test_code_example_clone_is_independent() {
    let original = CodeExample::new("original", "Original", "Desc", "code", "output");

    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);

    let modified = original.with_hints(vec!["hint".to_string()]);
    assert!(modified.hints.is_some());
    assert!(cloned.hints.is_none());
}

#[test]
fn test_code_example_multiple_hints() {
    let hints = vec![
        "First hint".to_string(),
        "Second hint".to_string(),
        "Third hint".to_string(),
    ];

    let example =
        CodeExample::new("test", "Test", "Desc", "code", "output").with_hints(hints.clone());

    assert_eq!(example.hints.unwrap(), hints);
}

#[test]
fn test_code_example_solution_is_complete() {
    let example = CodeExample::new("test", "Test", "Desc", "template", "output")
        .with_solution("full solution here");

    let solution = example.solution.unwrap();
    assert_eq!(solution, "full solution here");
    assert!(solution.len() >= example.code_template.len());
}

#[test]
fn test_difficulty_levels_are_distinct() {
    let beginner = Difficulty::Beginner;
    let intermediate = Difficulty::Intermediate;
    let advanced = Difficulty::Advanced;

    assert_ne!(beginner, intermediate);
    assert_ne!(intermediate, advanced);
    assert_ne!(beginner, advanced);
}

#[test]
fn test_example_default_difficulty_is_beginner() {
    let example = CodeExample::new("test", "Test", "Desc", "code", "output");

    assert_eq!(example.difficulty, Difficulty::Beginner);
}
