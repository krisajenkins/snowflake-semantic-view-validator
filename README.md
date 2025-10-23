# Snowflake Semantic View Validator (ssvv)

A command-line tool to validate Snowflake semantic model YAML files against the [official Snowflake specification](https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec).

## Features

- ✅ **Comprehensive Validation**: Validates semantic model YAML files against Snowflake's specification
- 📊 **Beautiful Output**: Displays a neatly-formatted summary of valid models
- 🐛 **Helpful Error Messages**: Shows detailed errors with line numbers and actionable advice
- 🎨 **Color-Coded Output**: Uses colors to make output easy to read and understand
- ⚡ **Fast**: Written in Rust for maximum performance
- 🧪 **Well-Tested**: Includes comprehensive test suite with golden master testing approach

## Installation

### Using Nix (Recommended)

If you have Nix with flakes enabled:

```bash
nix build .#ssvv
./result/bin/ssvv
```

Or install it to your profile:

```bash
nix profile install .#ssvv
```

### Using Cargo

```bash
cargo build --release
./target/release/ssvv
```

## Usage

### Validate a semantic model file

```bash
ssvv <file.yaml>
```

Example:

```bash
ssvv speedrun.yaml
```

### Show help

```bash
ssvv
# or
ssvv --help
```

## Example Output

### Valid File

When validating a valid semantic model file, you'll see a comprehensive summary:

```
════════════════════════════════════════════════════════════════════════════════
  SEMANTIC MODEL VALIDATION SUMMARY
════════════════════════════════════════════════════════════════════════════════

Name: SPEEDRUN
Description: This semantic model describes the Speedrun dataset...

TABLES
────────────────────────────────────────────────────────────────────────────────
  • GAMES
    Location: KJ_SPEEDRUN.PUBLIC.GAMES
    Dimensions: 16
    Time Dimensions: 2
    Facts: 0

  • LEADERBOARDS
    Location: KJ_SPEEDRUN.PUBLIC.LEADERBOARDS
    Dimensions: 9
    Time Dimensions: 2
    Facts: 4

RELATIONSHIPS
────────────────────────────────────────────────────────────────────────────────
  No relationships defined

VERIFIED QUERIES
────────────────────────────────────────────────────────────────────────────────
  No verified queries defined

════════════════════════════════════════════════════════════════════════════════
✓ Validation successful!
════════════════════════════════════════════════════════════════════════════════
```

### Invalid File

When validating an invalid file, you'll see helpful error messages:

```
════════════════════════════════════════════════════════════════════════════════
  VALIDATION ERROR
════════════════════════════════════════════════════════════════════════════════

✗ Failed to parse YAML file: invalid.yaml

TIP:
  Check the YAML syntax at the indicated line and column.
  Common issues include:
    • Incorrect indentation (use spaces, not tabs)
    • Missing colons after keys
    • Unquoted strings containing special characters
    • Missing required fields

════════════════════════════════════════════════════════════════════════════════
```

## Semantic Model Specification

The tool validates semantic models according to the [official Snowflake specification](https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec). A valid semantic model must include:

### Required Fields

- `name`: The name of the semantic model
- `description`: A description of what the model represents
- `tables`: An array of at least one table definition

### Table Structure

Each table must have:

- `name`: The table name
- `base_table`: Database location with `database`, `schema`, and `table` fields
- At least one of:
  - `dimensions`: Column definitions for dimensional data
  - `time_dimensions`: Column definitions for time-based data
  - `facts`: Column definitions for measurable/aggregatable data
  - `metrics`: Column definitions for aggregate metrics

### Optional Fields

#### Model Level
- `comments`: Additional comments about the model
- `relationships`: Defines how tables relate to each other via foreign keys
- `verified_queries`: Pre-defined questions and verified SQL queries (Verified Query Repository)
- `custom_instructions`: Custom instructions for Cortex Analyst
- `metrics`: Model-scoped derived metrics

#### Table Level
- `description`: Description of the table
- `synonyms`: Alternative names for the table
- `primary_key`: Primary key columns (required if table is used in relationships)
- `filters`: Predefined filters for the table

#### Column Level (Dimensions, Time Dimensions, Facts, Metrics, Filters)
- `synonyms`: Alternative names/phrases for the column
- `description`: Description of what the column represents
- `unique`: Boolean indicating if values are unique
- `sample_values`: Example values
- `access_modifier`: For facts and metrics - `public_access` or `private_access`
- `is_enum`: For dimensions - indicates if sample_values is exhaustive
- `cortex_search_service`: For dimensions - Cortex Search Service configuration

### Relationships

Relationships define joins between tables and must include:
- `name`: Unique identifier for the relationship
- `left_table`: Source table name
- `right_table`: Target table name
- `relationship_columns`: Array of column pairs to join on
  - `left_column`: Column name in left table
  - `right_column`: Column name in right table
- `join_type`: Either `left_outer` or `inner`
- `relationship_type`: Either `many_to_one` or `one_to_one`

### Spec Compliance

This validator is compliant with the official Snowflake Cortex Analyst semantic model specification as of October 2025. It supports all required and optional fields defined in the spec, including:

✅ Primary keys for relationship definitions  
✅ Synonyms for all entity types  
✅ Access modifiers for facts and metrics  
✅ Model-level derived metrics  
✅ Verified Query Repository (VQR)  
✅ Cortex Search Service integration  
✅ Complete relationship structure with column mappings

## Development

### Building

```bash
cargo build
```

### Running Tests

The project uses a "golden master" testing approach with example YAML files and their expected outputs:

```bash
cargo test
```

Test fixtures are located in `tests/fixtures/`:
- `*.yaml` - Input test files
- `*.expected` - Expected output for each test case

To regenerate expected outputs after making changes:

```bash
cargo build --release
./target/release/ssvv tests/fixtures/valid_basic.yaml > tests/fixtures/valid_basic.expected
./target/release/ssvv tests/fixtures/valid_with_relationships.yaml > tests/fixtures/valid_with_relationships.expected
```

### Project Structure

```
.
├── src/
│   └── main.rs           # Main application code
├── tests/
│   ├── integration_test.rs   # Integration tests
│   └── fixtures/             # Test fixtures and expected outputs
├── Cargo.toml            # Rust dependencies
├── flake.nix             # Nix build configuration
├── speedrun.yaml         # Example semantic model
└── README.md             # This file
```

## Dependencies

- [clap](https://crates.io/crates/clap) - Command-line argument parsing
- [serde](https://crates.io/crates/serde) - Serialization/deserialization
- [serde_yaml](https://crates.io/crates/serde_yaml) - YAML parsing
- [anyhow](https://crates.io/crates/anyhow) - Error handling
- [colored](https://crates.io/crates/colored) - Terminal colors

## License

This project is provided as-is for validating Snowflake semantic model files.

## Contributing

Contributions are welcome! Please ensure all tests pass before submitting a pull request:

```bash
cargo test
cargo fmt
cargo clippy
```
