# Snowflake Semantic View Validator (ssvv)

A command-line tool to validate Snowflake semantic model YAML files against the [official Snowflake specification](https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec).

## Usage

### Validate a semantic model file

```bash
ssvv <file.yaml>
```

Example:

```bash
ssvv speedrun.yaml

================================================================================
  SEMANTIC MODEL VALIDATION SUMMARY
================================================================================

Name: speedrun_semantic_model
Description: Semantic model for speedrun data including games, runs, users, and leaderboards.

IMPORTANT: Data Freshness
- The database contains historical speedrun data
- Before using time-based filters, check the actual date range with:
  SELECT MAX(date) as most_recent_run, MIN(date) as oldest_run FROM KJ_SPEEDRUN.PUBLIC.runs
- Adjust time filters based on available data rather than CURRENT_DATE() if data is not current


TABLES (17)
--------------------------------------------------------------------------------
Name          | Location                         | Dimensions | Time | Facts | Metrics | Filters
--------------|----------------------------------|------------|------|-------|---------|--------
games         | KJ_SPEEDRUN.PUBLIC.games         |         16 |    2 |     0 |       0 |       4
runs          | KJ_SPEEDRUN.PUBLIC.runs          |         11 |    2 |     4 |       0 |       6
users         | KJ_SPEEDRUN.PUBLIC.users         |          3 |    1 |     0 |       0 |       0
...

RELATIONSHIPS (19)
--------------------------------------------------------------------------------
Name                         | Join Type  | Left Table    | Right Table   | Type        | Columns                                                   
-----------------------------|------------|---------------|---------------|-------------|-----------------------------------------------------------
runs_to_games                | left_outer | runs          | games         | many_to_one | game_id = game_id                                         
runs_to_categories           | left_outer | runs          | categories    | many_to_one | category_id = category_id                                 
...

VERIFIED QUERIES (21)
--------------------------------------------------------------------------------
Name                                | Question                                                                                
------------------------------------|-----------------------------------------------------------------------------------------
genre_popularity                    | What are the game genres we have listed, and how popular are they?                      
total_individual_level_runs         | How many individual level runs are there across all games?                              
...

CUSTOM INSTRUCTIONS
--------------------------------------------------------------------------------
  module_custom_instructions:
    sql_generation:
      ## Query Construction Guidelines
      
      1. **Always use fully qualified table names**: All tables must be referenced as `KJ_SPEEDRUN.PUBLIC.table_name`
      ...

================================================================================
* Validation successful!
================================================================================
```

### Show help

```bash
ssvv
# or
ssvv --help
```

## Building & Installing

### Using Cargo

```bash
cargo build

cargo install --path .
```

### Using Nix

If you have Nix with flakes enabled:

```bash
nix build .#ssvv
ls ./result/bin/ssvv
```

Or install it to your profile:

```bash
nix profile add .
```

## Example Output

### Valid File

When validating a valid semantic model file, you'll see a comprehensive summary:


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
