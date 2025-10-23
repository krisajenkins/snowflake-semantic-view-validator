use clap::Parser;
use colored::*;
use snowflake_semantic_view_validator::{validate_file, format_error, format_success};

/// Snowflake Semantic View Validator (ssvv)
/// 
/// Validates Snowflake semantic model YAML files against the official specification.
#[derive(Parser, Debug)]
#[command(name = "ssvv")]
#[command(about = "Snowflake Semantic View Validator", long_about = None)]
struct Cli {
    /// Path to the semantic model YAML file to validate
    file: Option<String>,
}

fn format_help() -> String {
    let mut output = String::new();
    
    output.push_str("Snowflake Semantic View Validator (ssvv)\n");
    output.push('\n');
    output.push_str("A tool to validate Snowflake semantic model YAML files against the official specification.\n");
    output.push('\n');
    output.push_str("USAGE:\n");
    output.push_str("  ssvv <file.yaml>    Validate a semantic model file\n");
    output.push_str("  ssvv --help         Show this help message\n");
    output.push('\n');
    output.push_str("DESCRIPTION:\n");
    output.push_str("  This tool validates Snowflake semantic model YAML files according to the\n");
    output.push_str("  specification at:\n");
    output.push_str("  https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec\n");
    output.push('\n');
    output.push_str("  If the file is valid, it displays a comprehensive summary of the model.\n");
    output.push_str("  If the file is invalid, it shows detailed error messages with line numbers\n");
    output.push_str("  and helpful advice on how to fix the issues.\n");
    output.push('\n');
    output.push_str("EXAMPLES:\n");
    output.push_str("  ssvv speedrun.yaml\n");
    output.push_str("  ssvv my-semantic-model.yml\n");
    
    output
}

fn print_colored(text: &str, is_error: bool) {
    for line in text.lines() {
        if line.contains("═") {
            if is_error {
                println!("{}", line.bright_red());
            } else {
                println!("{}", line.bright_blue());
            }
        } else if line.contains("VALIDATION ERROR") {
            println!("{}", line.bright_red().bold());
        } else if line.contains("SEMANTIC MODEL VALIDATION SUMMARY") {
            println!("{}", line.bright_blue().bold());
        } else if line.starts_with("✗") {
            println!("{}", line.bright_red().bold());
        } else if line.starts_with("✓") {
            println!("{}", line.bright_green().bold());
        } else if line.starts_with("Name:") || line.starts_with("Description:") {
            if let Some(colon_pos) = line.find(':') {
                let (key, value) = line.split_at(colon_pos + 1);
                print!("{}", key.bright_green().bold());
                println!("{}", value);
            } else {
                println!("{}", line);
            }
        } else if line == "TABLES" || line == "RELATIONSHIPS" || line == "CUSTOM QUERIES" {
            println!("{}", line.bright_yellow().bold());
        } else if line.contains("─") {
            println!("{}", line.bright_black());
        } else if line.starts_with("  • ") {
            let rest = line.strip_prefix("  • ").unwrap_or(line);
            print!("  {} ", "•".bright_cyan());
            println!("{}", rest.bold());
        } else if line.starts_with("    ") && (line.contains("Location:") || line.contains("Dimensions:") || line.contains("Time Dimensions:") || line.contains("Facts:")) {
            println!("{}", line.dimmed());
        } else if line == "TIP:" {
            println!("{}", line.bright_yellow().bold());
        } else {
            println!("{}", line);
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        Some(path) => {
            match validate_file(&path) {
                Ok(model) => {
                    let output = format_success(&model);
                    print_colored(&output, false);
                }
                Err(e) => {
                    let output = format_error(&e);
                    print_colored(&output, true);
                    std::process::exit(1);
                }
            }
        }
        None => {
            let help = format_help();
            print_colored(&help, false);
        }
    }
}
