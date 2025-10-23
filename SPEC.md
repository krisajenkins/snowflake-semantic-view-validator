This is a Rust project. The binary should be called ssvv (for "Snowflake Semantic View Validator"). 

If you call `ssvv <somefile.yaml>` it will validate it against Snowflake's spec (https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec#specification). If the file is valid, the program prints a neatly-presented summary of your document. If it's invalid, it shows a helpful error message, complete with line numbers (where appropriate) and user-friendly advice on how to fix it.

If you call `ssvv` with any other arguments, it prints a help message and description of how the project works.

You'll see there's a sample file in `speedrun.yml`. It's valid, but not very thorough. The output should show that the file's valid, and it should show that there are zero relationships defined, and zero custom queries.

Use `clap` for parsing Rust command line arguments. Use `serde` for deserialisation.

We need thorough tests. I want there to be a directory of example YAML files, and a twin example output file. There should be a test suite that runs each example yaml file and check the output exactly matches the expected out text file. It's a "golden master" testing approach.

We need a README.md explaining what the project is, how to use it, and how to build it.

We also need a build target added to the nix flake, such that `nix build .#ssvv` will create the binary. I believe there's a Nix `buildCargoPackage` tool that will help with this.

Ask me any clarifying questions you need to, then begin.
