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
...
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
