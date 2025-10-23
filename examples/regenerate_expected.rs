use snowflake_semantic_view_validator::{format_error, format_success, validate_file};
use std::fs;

fn main() {
    let fixtures = vec![
        ("valid_basic", true),
        ("valid_with_relationships", true),
        ("valid_module_custom_instructions", true),
        ("invalid_missing_name", false),
        ("invalid_no_tables", false),
        ("invalid_yaml_syntax", false),
        ("invalid_empty_module_instructions", false),
    ];

    for (name, is_valid) in fixtures {
        let path = format!("tests/fixtures/{}.yaml", name);
        let output_path = format!("tests/fixtures/{}.expected", name);

        let result = validate_file(&path);
        let doc = if is_valid {
            format_success(&result.unwrap().model)
        } else {
            format_error(&result.unwrap_err())
        };

        let output = doc.render_plain();

        fs::write(&output_path, output).expect(&format!("Failed to write {}", output_path));
        println!("Generated {}", output_path);
    }
}
