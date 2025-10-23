mod colored_doc;

pub use colored_doc::{
    color_spec, dimmed_spec, heading, separator, subheading, Alignment, Cell, ColoredDoc, Column,
    Table as TableRenderer,
};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_custom_instructions: Option<ModuleCustomInstructions>,
    #[serde(default)]
    pub metrics: Vec<Metric>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModuleCustomInstructions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_categorization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_generation: Option<String>,
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

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub model: SemanticModel,
    pub warnings: Vec<ValidationWarning>,
}

/// Parse and validate a semantic model file
pub fn validate_file(path: impl AsRef<Path>) -> Result<ValidationResult, ValidationError> {
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

    // Check for warnings
    let mut warnings = Vec::new();

    // Check if custom_instructions is used without module_custom_instructions
    if model.custom_instructions.is_some() && model.module_custom_instructions.is_none() {
        let custom_instructions = model.custom_instructions.as_ref().unwrap();
        warnings.push(ValidationWarning {
            message: "The 'custom_instructions' field is deprecated. Consider migrating to 'module_custom_instructions'.".to_string(),
            suggestion: Some(format!(
                "Replace:\n  custom_instructions: |\n    {}\n\nWith:\n  module_custom_instructions:\n    sql_generation: |\n      {}",
                custom_instructions.lines().collect::<Vec<_>>().join("\n    "),
                custom_instructions.lines().collect::<Vec<_>>().join("\n      ")
            )),
        });
    }

    // Validate module_custom_instructions if present
    if let Some(ref module_instructions) = model.module_custom_instructions {
        if module_instructions.question_categorization.is_none()
            && module_instructions.sql_generation.is_none()
        {
            return Err(ValidationError {
                message: "'module_custom_instructions' must have at least one of 'question_categorization' or 'sql_generation' defined".to_string(),
                is_yaml_error: false,
            });
        }
    }

    Ok(ValidationResult { model, warnings })
}

/// Format a validation error as a ColoredDoc
pub fn format_error(error: &ValidationError) -> ColoredDoc {
    heading("VALIDATION ERROR", Color::Red)
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            format!("* {}", error.message),
            color_spec(Color::Red, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::line())
        .append(if error.is_yaml_error {
            ColoredDoc::concat(vec![
                ColoredDoc::colored_text("TIP:", color_spec(Color::Yellow, true)),
                ColoredDoc::line(),
                ColoredDoc::text("  Check the YAML syntax at the indicated line and column."),
                ColoredDoc::line(),
                ColoredDoc::text("  Common issues include:"),
                ColoredDoc::line(),
                ColoredDoc::text("    * Incorrect indentation (use spaces, not tabs)"),
                ColoredDoc::line(),
                ColoredDoc::text("    * Missing colons after keys"),
                ColoredDoc::line(),
                ColoredDoc::text("    * Unquoted strings containing special characters"),
                ColoredDoc::line(),
                ColoredDoc::text("    * Missing required fields"),
                ColoredDoc::line(),
                ColoredDoc::line(),
            ])
        } else {
            ColoredDoc::text("")
        })
        .append(separator("=", Color::Red))
}

/// Format warnings as a ColoredDoc
pub fn format_warnings(warnings: &[ValidationWarning]) -> ColoredDoc {
    if warnings.is_empty() {
        return ColoredDoc::text("");
    }

    let mut doc = heading("WARNINGS", Color::Yellow).append(ColoredDoc::line());

    for warning in warnings {
        doc = doc
            .append(ColoredDoc::colored_text(
                "* ",
                color_spec(Color::Yellow, true),
            ))
            .append(ColoredDoc::colored_text(
                &warning.message,
                color_spec(Color::Yellow, false),
            ))
            .append(ColoredDoc::line());

        if let Some(suggestion) = &warning.suggestion {
            doc = doc
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    "  Suggestion:",
                    color_spec(Color::Cyan, true),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!("  {}", suggestion.lines().collect::<Vec<_>>().join("\n  ")),
                    dimmed_spec(),
                ))
                .append(ColoredDoc::line());
        }
        doc = doc.append(ColoredDoc::line());
    }

    doc.append(separator("-", Color::Yellow))
        .append(ColoredDoc::line())
}

/// Count columns with descriptions across all tables
fn count_described_columns(model: &SemanticModel) -> (usize, usize) {
    let mut total = 0;
    let mut described = 0;

    for table in &model.tables {
        // Count dimensions
        for dim in &table.dimensions {
            total += 1;
            if dim.description.is_some() {
                described += 1;
            }
        }
        // Count time dimensions
        for time_dim in &table.time_dimensions {
            total += 1;
            if time_dim.description.is_some() {
                described += 1;
            }
        }
        // Count facts
        for fact in &table.facts {
            total += 1;
            if fact.description.is_some() {
                described += 1;
            }
        }
        // Count metrics
        for metric in &table.metrics {
            total += 1;
            if metric.description.is_some() {
                described += 1;
            }
        }
        // Count filters
        for filter in &table.filters {
            total += 1;
            if filter.description.is_some() {
                described += 1;
            }
        }
    }

    // Count model-level metrics
    for metric in &model.metrics {
        total += 1;
        if metric.description.is_some() {
            described += 1;
        }
    }

    (described, total)
}

/// Count columns with synonyms across all tables
fn count_aliased_columns(model: &SemanticModel) -> (usize, usize) {
    let mut total = 0;
    let mut aliased = 0;

    for table in &model.tables {
        // Count dimensions
        for dim in &table.dimensions {
            total += 1;
            if let Some(synonyms) = &dim.synonyms {
                if !synonyms.is_empty() {
                    aliased += 1;
                }
            }
        }
        // Count time dimensions
        for time_dim in &table.time_dimensions {
            total += 1;
            if let Some(synonyms) = &time_dim.synonyms {
                if !synonyms.is_empty() {
                    aliased += 1;
                }
            }
        }
        // Count facts
        for fact in &table.facts {
            total += 1;
            if let Some(synonyms) = &fact.synonyms {
                if !synonyms.is_empty() {
                    aliased += 1;
                }
            }
        }
        // Count metrics
        for metric in &table.metrics {
            total += 1;
            if let Some(synonyms) = &metric.synonyms {
                if !synonyms.is_empty() {
                    aliased += 1;
                }
            }
        }
        // Count filters
        for filter in &table.filters {
            total += 1;
            if let Some(synonyms) = &filter.synonyms {
                if !synonyms.is_empty() {
                    aliased += 1;
                }
            }
        }
    }

    // Count model-level metrics
    for metric in &model.metrics {
        total += 1;
        if let Some(synonyms) = &metric.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }

    (aliased, total)
}

/// Count columns with descriptions for a single table
fn count_table_described_columns(table: &Table) -> (usize, usize) {
    let mut total = 0;
    let mut described = 0;

    // Count dimensions
    for dim in &table.dimensions {
        total += 1;
        if dim.description.is_some() {
            described += 1;
        }
    }
    // Count time dimensions
    for time_dim in &table.time_dimensions {
        total += 1;
        if time_dim.description.is_some() {
            described += 1;
        }
    }
    // Count facts
    for fact in &table.facts {
        total += 1;
        if fact.description.is_some() {
            described += 1;
        }
    }
    // Count metrics
    for metric in &table.metrics {
        total += 1;
        if metric.description.is_some() {
            described += 1;
        }
    }
    // Count filters
    for filter in &table.filters {
        total += 1;
        if filter.description.is_some() {
            described += 1;
        }
    }

    (described, total)
}

/// Count columns with synonyms for a single table
fn count_table_aliased_columns(table: &Table) -> (usize, usize) {
    let mut total = 0;
    let mut aliased = 0;

    // Count dimensions
    for dim in &table.dimensions {
        total += 1;
        if let Some(synonyms) = &dim.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }
    // Count time dimensions
    for time_dim in &table.time_dimensions {
        total += 1;
        if let Some(synonyms) = &time_dim.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }
    // Count facts
    for fact in &table.facts {
        total += 1;
        if let Some(synonyms) = &fact.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }
    // Count metrics
    for metric in &table.metrics {
        total += 1;
        if let Some(synonyms) = &metric.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }
    // Count filters
    for filter in &table.filters {
        total += 1;
        if let Some(synonyms) = &filter.synonyms {
            if !synonyms.is_empty() {
                aliased += 1;
            }
        }
    }

    (aliased, total)
}

/// Format a successful validation result as a ColoredDoc
pub fn format_success(model: &SemanticModel) -> ColoredDoc {
    let mut doc = heading("SEMANTIC MODEL VALIDATION SUMMARY", Color::Blue)
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "Name:",
            color_spec(Color::Green, true),
        ))
        .append(ColoredDoc::text(format!(" {}", model.name)))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "Description:",
            color_spec(Color::Green, true),
        ))
        .append(ColoredDoc::text(format!(" {}", model.description)))
        .append(ColoredDoc::line())
        .append(ColoredDoc::line())
        .append(subheading(&format!("TABLES ({})", model.tables.len()), Color::Yellow));

    // Build table for tables section - column by column
    let mut name_col = Column::new("Name");
    let mut location_col = Column::new("Location");
    let mut dims_col = Column::new_aligned("Dimensions", Alignment::Right);
    let mut time_col = Column::new_aligned("Time", Alignment::Right);
    let mut facts_col = Column::new_aligned("Facts", Alignment::Right);
    let mut metrics_col = Column::new_aligned("Metrics", Alignment::Right);
    let mut filters_col = Column::new_aligned("Filters", Alignment::Right);
    let mut described_col = Column::new_aligned("Described", Alignment::Right);
    let mut aliased_col = Column::new_aligned("Aliased", Alignment::Right);

    for table_item in &model.tables {
        let (described_count, total_count) = count_table_described_columns(table_item);
        let (aliased_count, _) = count_table_aliased_columns(table_item);
        
        let described_pct = if total_count > 0 {
            (described_count as f64 / total_count as f64) * 100.0
        } else {
            0.0
        };
        let aliased_pct = if total_count > 0 {
            (aliased_count as f64 / total_count as f64) * 100.0
        } else {
            0.0
        };

        name_col = name_col.add_cell(Cell::text(&table_item.name));
        location_col = location_col.add_cell(Cell::text(format!(
            "{}.{}.{}",
            table_item.base_table.database,
            table_item.base_table.schema,
            table_item.base_table.table
        )));
        dims_col = dims_col.add_cell(Cell::text(table_item.dimensions.len().to_string()));
        time_col = time_col.add_cell(Cell::text(table_item.time_dimensions.len().to_string()));
        facts_col = facts_col.add_cell(Cell::text(table_item.facts.len().to_string()));
        metrics_col = metrics_col.add_cell(Cell::text(table_item.metrics.len().to_string()));
        filters_col = filters_col.add_cell(Cell::text(table_item.filters.len().to_string()));
        described_col = described_col.add_cell(Cell::text(format!("{:.0}%", described_pct)));
        aliased_col = aliased_col.add_cell(Cell::text(format!("{:.0}%", aliased_pct)));
    }

    let table_renderer = TableRenderer::new()
        .add_column(name_col)
        .add_column(location_col)
        .add_column(dims_col)
        .add_column(time_col)
        .add_column(facts_col)
        .add_column(metrics_col)
        .add_column(filters_col)
        .add_column(described_col)
        .add_column(aliased_col);

    doc = doc
        .append(table_renderer.render())
        .append(ColoredDoc::line());

    // Relationships section
    doc = doc.append(subheading(&format!("RELATIONSHIPS ({})", model.relationships.len()), Color::Yellow));

    if model.relationships.is_empty() {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No relationships defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        let mut name_col = Column::new("Name");
        let mut join_col = Column::new("Join Type");
        let mut left_col = Column::new("Left Table");
        let mut right_col = Column::new("Right Table");
        let mut type_col = Column::new("Type");
        let mut columns_col = Column::new("Columns");

        for rel in &model.relationships {
            let columns_str = rel
                .relationship_columns
                .iter()
                .map(|c| format!("{} = {}", c.left_column, c.right_column))
                .collect::<Vec<_>>()
                .join(", ");

            name_col = name_col.add_cell(Cell::text(&rel.name));
            join_col = join_col.add_cell(Cell::text(&rel.join_type));
            left_col = left_col.add_cell(Cell::text(&rel.left_table));
            right_col = right_col.add_cell(Cell::text(&rel.right_table));
            type_col = type_col.add_cell(Cell::text(&rel.relationship_type));
            columns_col = columns_col.add_cell(Cell::text(columns_str));
        }

        let relationships_table = TableRenderer::new()
            .add_column(name_col)
            .add_column(join_col)
            .add_column(left_col)
            .add_column(right_col)
            .add_column(type_col)
            .add_column(columns_col);

        doc = doc.append(relationships_table.render());
    }
    doc = doc.append(ColoredDoc::line());

    // Verified Queries section
    doc = doc.append(subheading(&format!("VERIFIED QUERIES ({})", model.verified_queries.len()), Color::Yellow));

    if model.verified_queries.is_empty() {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No verified queries defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        let mut name_col = Column::new("Name");
        let mut question_col = Column::new("Question");

        for query in &model.verified_queries {
            name_col = name_col.add_cell(Cell::text(&query.name));
            question_col = question_col.add_cell(Cell::text(&query.question));
        }

        let queries_table = TableRenderer::new()
            .add_column(name_col)
            .add_column(question_col);

        doc = doc.append(queries_table.render());
    }
    doc = doc.append(ColoredDoc::line());

    // Custom Instructions section
    doc = doc.append(subheading("CUSTOM INSTRUCTIONS", Color::Yellow));

    let has_any_instructions =
        model.custom_instructions.is_some() || model.module_custom_instructions.is_some();

    if !has_any_instructions {
        doc = doc
            .append(ColoredDoc::colored_text(
                "  No custom instructions defined",
                dimmed_spec(),
            ))
            .append(ColoredDoc::line());
    } else {
        // Show legacy custom_instructions if present
        if let Some(instructions) = &model.custom_instructions {
            doc = doc
                .append(ColoredDoc::colored_text(
                    "  [DEPRECATED] custom_instructions:",
                    color_spec(Color::Yellow, true),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!(
                        "    {}",
                        instructions.lines().collect::<Vec<_>>().join("\n    ")
                    ),
                    dimmed_spec(),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    "  MIGRATION NEEDED:",
                    color_spec(Color::Cyan, true),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    "  Replace the above with:",
                    color_spec(Color::Cyan, false),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    "  module_custom_instructions:",
                    color_spec(Color::Green, false),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    "    sql_generation: |",
                    color_spec(Color::Green, false),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::colored_text(
                    format!(
                        "      {}",
                        instructions.lines().collect::<Vec<_>>().join("\n      ")
                    ),
                    color_spec(Color::Green, false),
                ))
                .append(ColoredDoc::line())
                .append(ColoredDoc::line());
        }

        // Show module_custom_instructions if present
        if let Some(module_instructions) = &model.module_custom_instructions {
            doc = doc
                .append(ColoredDoc::colored_text(
                    "  module_custom_instructions:",
                    color_spec(Color::Cyan, true),
                ))
                .append(ColoredDoc::line());

            if let Some(question_cat) = &module_instructions.question_categorization {
                doc = doc
                    .append(ColoredDoc::colored_text(
                        "    question_categorization:",
                        color_spec(Color::White, true),
                    ))
                    .append(ColoredDoc::line())
                    .append(ColoredDoc::colored_text(
                        format!(
                            "      {}",
                            question_cat.lines().collect::<Vec<_>>().join("\n      ")
                        ),
                        dimmed_spec(),
                    ))
                    .append(ColoredDoc::line())
                    .append(ColoredDoc::line());
            }

            if let Some(sql_gen) = &module_instructions.sql_generation {
                doc = doc
                    .append(ColoredDoc::colored_text(
                        "    sql_generation:",
                        color_spec(Color::White, true),
                    ))
                    .append(ColoredDoc::line())
                    .append(ColoredDoc::colored_text(
                        format!(
                            "      {}",
                            sql_gen.lines().collect::<Vec<_>>().join("\n      ")
                        ),
                        dimmed_spec(),
                    ))
                    .append(ColoredDoc::line());
            }
        }
    }
    doc = doc.append(ColoredDoc::line());

    // Data Quality Metrics section
    let (described_count, total_columns) = count_described_columns(model);
    let (aliased_count, _) = count_aliased_columns(model);
    let described_pct = if total_columns > 0 {
        (described_count as f64 / total_columns as f64) * 100.0
    } else {
        0.0
    };
    let aliased_pct = if total_columns > 0 {
        (aliased_count as f64 / total_columns as f64) * 100.0
    } else {
        0.0
    };

    doc = doc.append(subheading("DATA QUALITY METRICS", Color::Yellow));

    // Determine color based on percentage
    let described_color = if described_pct >= 80.0 {
        Color::Green
    } else if described_pct >= 50.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    let aliased_color = if aliased_pct >= 60.0 {
        Color::Green
    } else if aliased_pct >= 30.0 {
        Color::Yellow
    } else {
        Color::Red
    };

    doc = doc
        .append(ColoredDoc::colored_text(
            "  Described Columns:",
            color_spec(Color::Cyan, true),
        ))
        .append(ColoredDoc::text(format!(" {} / {} ", described_count, total_columns)))
        .append(ColoredDoc::colored_text(
            format!("({:.1}%)", described_pct),
            color_spec(described_color, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "  Aliased Columns:",
            color_spec(Color::Cyan, true),
        ))
        .append(ColoredDoc::text(format!(" {} / {} ", aliased_count, total_columns)))
        .append(ColoredDoc::colored_text(
            format!("({:.1}%)", aliased_pct),
            color_spec(aliased_color, true),
        ))
        .append(ColoredDoc::line())
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "  TIP:",
            color_spec(Color::Magenta, true),
        ))
        .append(ColoredDoc::text(" Descriptions and synonyms help LLMs understand your data model better."))
        .append(ColoredDoc::line())
        .append(ColoredDoc::colored_text(
            "       ",
            color_spec(Color::Magenta, true),
        ))
        .append(ColoredDoc::text("Aim for 80%+ described columns and 60%+ aliased columns for optimal results."))
        .append(ColoredDoc::line())
        .append(ColoredDoc::line());

    // Success footer
    doc.append(separator("=", Color::Blue))
        .append(ColoredDoc::colored_text(
            "*",
            color_spec(Color::Green, true),
        ))
        .append(ColoredDoc::text(" "))
        .append(ColoredDoc::colored_text(
            "Validation successful!",
            color_spec(Color::Green, false),
        ))
        .append(ColoredDoc::line())
        .append(separator("=", Color::Blue))
}
