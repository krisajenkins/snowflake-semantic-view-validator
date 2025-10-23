# AGENTS.md

This file provides guidance for AI coding agents working on the Snowflake Semantic View Validator (ssvv) project.

## Project Overview

**ssvv** is a command-line tool written in Rust that validates Snowflake semantic model YAML files against the [official Snowflake specification](https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec).

**Key Features:**
- Parses and validates Snowflake semantic model YAML files
- Provides detailed, colorized output with helpful error messages
- Displays comprehensive summaries for valid models
- Uses a "golden master" testing approach with fixture files

**Project Structure:**
```
.
??? src/
?   ??? main.rs           # CLI entry point and argument parsing
?   ??? lib.rs            # Core validation logic and data structures
?   ??? colored_doc.rs    # Terminal output formatting utilities
??? tests/
?   ??? integration_test.rs   # Integration tests
?   ??? fixtures/             # Test YAML files and expected outputs
??? examples/
?   ??? complete_example.yaml # Comprehensive example file
??? Cargo.toml            # Rust dependencies
??? flake.nix             # Nix build configuration
??? speedrun.yaml         # Example semantic model
```

## Development Environment

### Prerequisites
- Rust toolchain (cargo, rustc, rustfmt, clippy)
- Nix (optional, for reproducible builds)

### Setup Commands

**Using Cargo:**
```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

**Using Nix (with flakes):**
```bash
# Build the binary
nix build .#ssvv

# Run without installing
nix run . -- <file.yaml>

# Enter development shell
nix develop

# Install to profile
nix profile install .
```

### Running the Tool

```bash
# Validate a file
ssvv <file.yaml>

# Show help
ssvv
ssvv --help
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_valid_basic
```

### Test Structure

The project uses a **golden master** testing approach:
- Test fixtures are in `tests/fixtures/`
- Each test has two files:
  - `*.yaml` - Input test file
  - `*.expected` - Expected output (plain text, no ANSI colors)
- Tests validate that actual output matches expected output exactly

### Regenerating Expected Outputs

When you make intentional changes to output formatting:

```bash
# Build release version first
cargo build --release

# Regenerate expected outputs
./target/release/ssvv tests/fixtures/valid_basic.yaml > tests/fixtures/valid_basic.expected
./target/release/ssvv tests/fixtures/valid_with_relationships.yaml > tests/fixtures/valid_with_relationships.expected
./target/release/ssvv tests/fixtures/invalid_missing_name.yaml 2> tests/fixtures/invalid_missing_name.expected
./target/release/ssvv tests/fixtures/invalid_no_tables.yaml 2> tests/fixtures/invalid_no_tables.expected
./target/release/ssvv tests/fixtures/invalid_yaml_syntax.yaml 2> tests/fixtures/invalid_yaml_syntax.expected
```

**Note:** Error outputs go to stderr, so use `2>` for invalid test cases.

Alternatively, use the helper script:
```bash
cargo run --example regenerate_expected
```

## Code Style and Conventions

### Rust Style
- Follow standard Rust formatting (enforced by `rustfmt`)
- Use descriptive variable names
- Prefer explicit types for public APIs
- Use `Option<T>` for optional fields in structs
- Use `Result<T, E>` for operations that can fail

### Formatting and Linting

```bash
# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check

# Run clippy linter
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Serde Conventions

- Use `#[serde(skip_serializing_if = "Option::is_none")]` for optional fields
- Use `#[serde(default)]` for fields that should default to empty collections
- Field names in structs use `snake_case`, YAML uses `snake_case`

### Output Formatting

- Use the `ColoredDoc` API for all terminal output
- Success messages use blue/green colors
- Error messages use red/yellow colors
- Dimmed text for secondary information
- 80-character separator lines using `?` and `?`

## Architecture Notes

### Data Flow

1. **CLI Parsing** (`main.rs`): Uses `clap` to parse command-line arguments
2. **File Reading** (`lib.rs`): Reads YAML file from filesystem
3. **YAML Parsing** (`lib.rs`): Uses `serde_yaml` to deserialize into `SemanticModel` struct
4. **Validation** (`lib.rs`): Validates required fields and business rules
5. **Output Formatting** (`lib.rs`): Formats success or error output using `ColoredDoc`
6. **Rendering** (`colored_doc.rs`): Renders to terminal with or without colors

### Key Types

- `SemanticModel`: Root type representing the entire semantic model
- `Table`: Represents a logical table in the model
- `Dimension`, `TimeDimension`, `Fact`, `Metric`: Column-level definitions
- `Relationship`: Defines joins between tables
- `VerifiedQuery`: Example queries with expected results
- `ValidationError`: Custom error type with helpful messages

### Validation Rules

Current validation checks:
- Model must have a non-empty `name`
- Model must have at least one table
- Each table must have a non-empty `name`
- Each table must have at least one dimension, time_dimension, fact, or metric

**Important:** See `SPEC_ANALYSIS.md` for a comprehensive list of validation rules from the Snowflake spec that are NOT yet implemented.

## Common Tasks

### Adding a New Field to the Data Model

1. Add the field to the appropriate struct in `src/lib.rs`
2. Add `#[serde(skip_serializing_if = "Option::is_none")]` if optional
3. Update the `format_success` function to display the new field
4. Add test coverage in `tests/fixtures/`
5. Update expected outputs
6. Run tests to verify

### Adding a New Validation Rule

1. Add validation logic in the `validate_file` function in `src/lib.rs`
2. Create test fixtures for valid and invalid cases
3. Add test cases in `tests/integration_test.rs`
4. Generate expected outputs
5. Run tests to verify

### Modifying Output Format

1. Update the `format_success` or `format_error` function in `src/lib.rs`
2. Rebuild: `cargo build --release`
3. Regenerate ALL expected outputs (see "Regenerating Expected Outputs" above)
4. Run tests to verify changes

## Dependencies

- **clap** (v4.5): Command-line argument parsing with derive macros
- **serde** (v1.0): Serialization/deserialization framework
- **serde_yaml** (v0.9): YAML parsing and serialization
- **termcolor** (v1.4): Cross-platform terminal color support

## Known Issues and Future Work

See `SPEC_ANALYSIS.md` for a detailed analysis of:
- Fields that need to be added to match the Snowflake spec
- Validation rules that should be implemented
- Current discrepancies between the validator and the official spec

### High Priority Items

1. **Advanced validation rules**: Implement circular reference detection, granularity validation, expression validation
2. **Better error messages**: Include line numbers for YAML parsing errors
3. **Performance**: Optimize for large semantic models
4. **Additional output formats**: JSON, structured output for CI/CD

## Commit Message Guidelines

- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- Reference issue numbers when applicable
- Keep first line under 72 characters
- Examples:
  - `Add support for cortex_search_service field`
  - `Fix validation for empty table names`
  - `Update README with new installation instructions`

## Pull Request Guidelines

- Run all checks before submitting:
  ```bash
  cargo fmt --check
  cargo clippy
  cargo test
  ```
- Update tests and expected outputs if needed
- Update documentation (README.md, SPEC.md, etc.) if adding features
- Provide clear description of changes and motivation
- Include example usage if adding new features

## Security Considerations

- The tool reads files from the filesystem - validate paths to prevent directory traversal
- YAML parsing uses `serde_yaml` which is memory-safe
- No network access or external dependencies at runtime
- No sensitive data is logged or stored

## Resources

- [Snowflake Semantic Model Spec](https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde Documentation](https://serde.rs/)
- [Clap Documentation](https://docs.rs/clap/)

## Quick Reference

### File Locations
- Main CLI: `src/main.rs`
- Core logic: `src/lib.rs`
- Output formatting: `src/colored_doc.rs`
- Tests: `tests/integration_test.rs`
- Test fixtures: `tests/fixtures/`
- Example files: `examples/` and `speedrun.yaml`

### Common Commands
```bash
# Development cycle
cargo fmt && cargo clippy && cargo test

# Build and test
cargo build --release && ./target/release/ssvv speedrun.yaml

# Nix build
nix build .#ssvv && ./result/bin/ssvv speedrun.yaml

# Run checks
nix flake check
```
