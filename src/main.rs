use clap::Parser;
use snowflake_semantic_view_validator::{
    format_error, format_success, format_warnings, validate_file, ColoredDoc,
};
use termcolor::{ColorChoice, StandardStream};

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

fn format_help() -> ColoredDoc {
    ColoredDoc::concat(vec![
        ColoredDoc::text("Snowflake Semantic View Validator (ssvv)"),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::text("A tool to validate Snowflake semantic model YAML files against the official specification."),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::text("USAGE:"),
        ColoredDoc::line(),
        ColoredDoc::text("  ssvv <file.yaml>    Validate a semantic model file"),
        ColoredDoc::line(),
        ColoredDoc::text("  ssvv --help         Show this help message"),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::text("DESCRIPTION:"),
        ColoredDoc::line(),
        ColoredDoc::text("  This tool validates Snowflake semantic model YAML files according to the"),
        ColoredDoc::line(),
        ColoredDoc::text("  specification at:"),
        ColoredDoc::line(),
        ColoredDoc::text("  https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst/semantic-model-spec"),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::text("  If the file is valid, it displays a comprehensive summary of the model."),
        ColoredDoc::line(),
        ColoredDoc::text("  If the file is invalid, it shows detailed error messages with line numbers"),
        ColoredDoc::line(),
        ColoredDoc::text("  and helpful advice on how to fix the issues."),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::text("EXAMPLES:"),
        ColoredDoc::line(),
        ColoredDoc::text("  ssvv speedrun.yaml"),
        ColoredDoc::line(),
        ColoredDoc::text("  ssvv my-semantic-model.yml"),
        ColoredDoc::line(),
    ])
}

fn main() {
    let cli = Cli::parse();

    match cli.file {
        Some(path) => match validate_file(&path) {
            Ok(result) => {
                let mut stdout = StandardStream::stdout(ColorChoice::Auto);

                // Show warnings first if any
                if !result.warnings.is_empty() {
                    let warnings_doc = format_warnings(&result.warnings);
                    warnings_doc.render_colored(&mut stdout).unwrap();
                }

                // Then show the success summary
                let doc = format_success(&result.model);
                doc.render_colored(&mut stdout).unwrap();
            }
            Err(e) => {
                let doc = format_error(&e);
                let mut stderr = StandardStream::stderr(ColorChoice::Auto);
                doc.render_colored(&mut stderr).unwrap();
                std::process::exit(1);
            }
        },
        None => {
            let help = format_help();
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            help.render_colored(&mut stdout).unwrap();
        }
    }
}
