use std::fs;
use std::path::PathBuf;

// Import the public functions from main.rs
use snowflake_semantic_view_validator::{format_error, format_success, validate_file};

fn get_fixture_path(fixture_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(format!("{}.yaml", fixture_name));
    path
}

fn get_expected_path(fixture_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(format!("{}.expected", fixture_name));
    path
}

fn run_valid_test_case(fixture_name: &str) {
    let fixture_path = get_fixture_path(fixture_name);
    let expected_path = get_expected_path(fixture_name);

    let result = validate_file(&fixture_path);
    assert!(
        result.is_ok(),
        "Test case '{}' should be valid, but got error: {:?}",
        fixture_name,
        result.err()
    );

    let validation_result = result.unwrap();
    let doc = format_success(&validation_result.model);
    let actual = doc.render_plain();

    let expected = fs::read_to_string(&expected_path).expect(&format!(
        "Failed to read expected output file: {:?}",
        expected_path
    ));

    assert_eq!(
        actual.trim(),
        expected.trim(),
        "Test case '{}' output mismatch.\n\nExpected:\n{}\n\nActual:\n{}",
        fixture_name,
        expected,
        actual
    );
}

fn run_invalid_test_case(fixture_name: &str) {
    let fixture_path = get_fixture_path(fixture_name);
    let expected_path = get_expected_path(fixture_name);

    let result = validate_file(&fixture_path);
    assert!(
        result.is_err(),
        "Test case '{}' should be invalid, but validation succeeded",
        fixture_name
    );

    let error = result.unwrap_err();
    let doc = format_error(&error);
    let actual = doc.render_plain();

    let expected = fs::read_to_string(&expected_path).expect(&format!(
        "Failed to read expected output file: {:?}",
        expected_path
    ));

    assert_eq!(
        actual.trim(),
        expected.trim(),
        "Test case '{}' output mismatch.\n\nExpected:\n{}\n\nActual:\n{}",
        fixture_name,
        expected,
        actual
    );
}

#[test]
fn test_valid_basic() {
    run_valid_test_case("valid_basic");
}

#[test]
fn test_valid_with_relationships() {
    run_valid_test_case("valid_with_relationships");
}

#[test]
fn test_invalid_missing_name() {
    run_invalid_test_case("invalid_missing_name");
}

#[test]
fn test_invalid_no_tables() {
    run_invalid_test_case("invalid_no_tables");
}

#[test]
fn test_invalid_yaml_syntax() {
    run_invalid_test_case("invalid_yaml_syntax");
}

#[test]
fn test_valid_module_custom_instructions() {
    run_valid_test_case("valid_module_custom_instructions");
}

#[test]
fn test_invalid_empty_module_instructions() {
    run_invalid_test_case("invalid_empty_module_instructions");
}
