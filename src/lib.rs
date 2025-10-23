mod colored_doc;

pub use colored_doc::{color_spec, dimmed_spec, ColoredDoc};

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;
use termcolor::Color;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SemanticModel {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    pub tables: Vec<Table>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(default)]
    pub verified_queries: Vec<VerifiedQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<String>,
    #[serde(default)]
    pub metrics: Vec<Metric>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    pub base_table: BaseTable,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<PrimaryKey>,
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
    #[serde(default)]
    pub time_dimensions: Vec<TimeDimension>,
    #[serde(default)]
    pub facts: Vec<Fact>,
    #[serde(default)]
    pub metrics: Vec<Metric>,
    #[serde(default)]
    pub filters: Vec<Filter>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BaseTable {
    pub database: String,
    pub schema: String,
    pub table: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrimaryKey {
    pub columns: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dimension {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cortex_search_service: Option<CortexSearchService>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeDimension {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_values: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Fact {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modifier: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metric {
    pub name: String,
    pub expr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_modifier: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filter {
    pub name: String,
    pub expr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relationship {
    pub name: String,
    pub left_table: String,
    pub right_table: String,
    pub relationship_columns: Vec<RelationshipColumn>,
    pub join_type: String,
    pub relationship_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelationshipColumn {
    pub left_column: String,
    pub right_column: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CortexSearchService {
    pub service: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub literal_column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerifiedQuery {
    pub name: String,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_result: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub message: String,
    pub is_yaml_error: bool,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Parse and validate a semantic model file
pub fn validate_file(path: impl AsRef<Path>) -> Result<SemanticModel, ValidationError> {
    let path = path.as_ref();

    let contents = fs::read_to_string(path).map_err(|e| ValidationError {
        message: format!("Failed to read file: {}", e),
        is_yaml_error: false,
    })?;

    let model: SemanticModel = serde_yaml::from_str(&contents).map_err(|e| ValidationError {
        message: format!("Failed to parse YAML file: {}", e),
        is_yaml_error: true,
    })?;

    // Basic validation
    if model.name.is_empty() {
        return Err(ValidationError {
            message: "Semantic model must have a non-empty 'name' field".to_string(),
            is_yaml_error: false,
        });
    }

    if model.tables.is_empty() {
        return Err(ValidationError {
            message: "Semantic model must have at least one table".to_string(),
            is_yaml_error: false,
        });
    }

    for (i, table) in model.tables.iter().enumerate() {
        if table.name.is_empty() {
            return Err(ValidationError {
                message: format!("Table at index {} must have a non-empty 'name' field", i),
                is_yaml_error: false,
            });
        }

        // Validate that each table has at least one dimension, time_dimension, fact, or metric
        if table.dimensions.is_empty()
            && table.time_dimensions.is_empty()
            && table.facts.is_empty()
            && table.metrics.is_empty()
        {
            return Err(ValidationError {
                message: format!(
                    "Table '{}' must have at least one dimension, time_dimension, fact, or metric",
                    table.name
                ),
                is_yaml_error: false,
            });
        }
    }

    Ok(model)
}

/// Format a validation error as a ColoredDoc
pub fn format_error(error: &ValidationError) -> ColoredDoc {
    ColoredDoc::concat(vec![
        ColoredDoc::colored_text("═".repeat(80), color_spec(Color::Red, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("  VALIDATION ERROR", color_spec(Color::Red, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("═".repeat(80), color_spec(Color::Red, true)),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::colored_text(format!("✗ {}", error.message), color_spec(Color::Red, true)),
        ColoredDoc::line(),
        ColoredDoc::line(),
    ])
    .append(if error.is_yaml_error {
        ColoredDoc::concat(vec![
            ColoredDoc::colored_text("TIP:", color_spec(Color::Yellow, true)),
            ColoredDoc::line(),
            ColoredDoc::text("  Check the YAML syntax at the indicated line and column."),
            ColoredDoc::line(),
            ColoredDoc::text("  Common issues include:"),
            ColoredDoc::line(),
            ColoredDoc::text("    • Incorrect indentation (use spaces, not tabs)"),
            ColoredDoc::line(),
            ColoredDoc::text("    • Missing colons after keys"),
            ColoredDoc::line(),
            ColoredDoc::text("    • Unquoted strings containing special characters"),
            ColoredDoc::line(),
            ColoredDoc::text("    • Missing required fields"),
            ColoredDoc::line(),
            ColoredDoc::line(),
        ])
    } else {
        ColoredDoc::text("")
    })
    .append(ColoredDoc::colored_text(
        "═".repeat(80),
        color_spec(Color::Red, true),
    ))
}

/// Format a successful validation result as a ColoredDoc
pub fn format_success(model: &SemanticModel) -> ColoredDoc {
    let mut doc = ColoredDoc::concat(vec![
        ColoredDoc::colored_text("═".repeat(80), color_spec(Color::Blue, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text(
            "  SEMANTIC MODEL VALIDATION SUMMARY",
            color_spec(Color::Blue, true),
        ),
        ColoredDoc::line(),
        ColoredDoc::colored_text("═".repeat(80), color_spec(Color::Blue, true)),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::colored_text("Name:", color_spec(Color::Green, true)),
        ColoredDoc::text(format!(" {}", model.name)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("Description:", color_spec(Color::Green, true)),
        ColoredDoc::text(format!(" {}", model.description)),
        ColoredDoc::line(),
        ColoredDoc::line(),
        ColoredDoc::colored_text("TABLES", color_spec(Color::Yellow, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("─".repeat(80), color_spec(Color::Black, true)),
        ColoredDoc::line(),
    ]);

    // Add tables
    for table in &model.tables {
        doc = doc
            .append(ColoredDoc::text("  "))
            .append(ColoredDoc::colored_text("•", color_spec(Color::Cyan, true)))
            .append(ColoredDoc::text(" "))
            .append(ColoredDoc::colored_text(
                &table.name,
                color_spec(Color::White, true),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!(
                    "    Location: {}.{}.{}",
                    table.base_table.database, table.base_table.schema, table.base_table.table
                ),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!("    Dimensions: {}", table.dimensions.len()),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!("    Time Dimensions: {}", table.time_dimensions.len()),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!("    Facts: {}", table.facts.len()),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!("    Metrics: {}", table.metrics.len()),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::colored_text(
                format!("    Filters: {}", table.filters.len()),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line())
            .append(ColoredDoc::line());
    }

    // Relationships section
    doc = doc
        .append(ColoredDoc::colored_text(
            "RELATIONSHIPS",
            color_spec(Color::Yellow, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "─".repeat(80),
            color_spec(Color::Black, true),
        ))
        .append(ColoredDoc::line());

    if model.relationships.is_empty() {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No relationships defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        for rel in &model.relationships {
            let columns_str = rel
                .relationship_columns
                .iter()
                .map(|c| format!("{} = {}", c.left_column, c.right_column))
                .collect::<Vec<_>>()
                .join(", ");
            doc = doc
                .append(ColoredDoc::text("  "))
                .append(ColoredDoc::colored_text("•", color_spec(Color::Cyan, true)))
                .append(ColoredDoc::text(" "))
                .append(ColoredDoc::colored_text(
                    &rel.name,
                    color_spec(Color::White, true),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!(
                        "    {} {} → {} ({})",
                        rel.join_type, rel.left_table, rel.right_table, rel.relationship_type
                    ),
                    dimmed_spec(),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!("    Columns: {}", columns_str),
                    dimmed_spec(),
                ))
                .append(ColoredDoc::line());
        }
    }
    doc = doc.append(ColoredDoc::line());

    // Verified Queries section
    doc = doc
        .append(ColoredDoc::colored_text(
            "VERIFIED QUERIES",
            color_spec(Color::Yellow, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "─".repeat(80),
            color_spec(Color::Black, true),
        ))
        .append(ColoredDoc::line());

    if model.verified_queries.is_empty() {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No verified queries defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        for query in &model.verified_queries {
            doc = doc
                .append(ColoredDoc::text("  "))
                .append(ColoredDoc::colored_text("•", color_spec(Color::Cyan, true)))
                .append(ColoredDoc::text(" "))
                .append(ColoredDoc::colored_text(
                    &query.name,
                    color_spec(Color::White, true),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!("    Question: {}", query.question),
                    dimmed_spec(),
                ))
                .append(ColoredDoc::line());
        }
    }
    doc = doc.append(ColoredDoc::line());

    // Custom Instructions section
    doc = doc
        .append(ColoredDoc::colored_text(
            "CUSTOM INSTRUCTIONS",
            color_spec(Color::Yellow, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "─".repeat(80),
            color_spec(Color::Black, true),
        ))
        .append(ColoredDoc::line());

    if let Some(instructions) = &model.custom_instructions {
        doc = doc
            .append(ColoredDoc::colored_text(
                format!("  {}", instructions),
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No custom instructions defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    }
    doc = doc.append(ColoredDoc::line());

    // Success footer
    doc.append(ColoredDoc::colored_text(
        "═".repeat(80),
        color_spec(Color::Blue, true),
    ))
    .append(ColoredDoc::line())
    .append(ColoredDoc::colored_text(
        "✓",
        color_spec(Color::Green, true),
    ))
    .append(ColoredDoc::text(" "))
    .append(ColoredDoc::colored_text(
        "Validation successful!",
        color_spec(Color::Green, false),
    ))
    .append(ColoredDoc::line())
    .append(ColoredDoc::colored_text(
        "═".repeat(80),
        color_spec(Color::Blue, true),
    ))
}
