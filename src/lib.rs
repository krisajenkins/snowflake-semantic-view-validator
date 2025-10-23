use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SemanticModel {
    pub name: String,
    pub description: String,
    pub tables: Vec<Table>,
    #[serde(default)]
    pub relationships: Vec<Relationship>,
    #[serde(default)]
    pub custom_queries: Vec<CustomQuery>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Table {
    pub name: String,
    pub base_table: BaseTable,
    #[serde(default)]
    pub dimensions: Vec<Dimension>,
    #[serde(default)]
    pub time_dimensions: Vec<TimeDimension>,
    #[serde(default)]
    pub facts: Vec<Fact>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BaseTable {
    pub database: String,
    pub schema: String,
    pub table: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dimension {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_values: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimeDimension {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Fact {
    pub name: String,
    pub expr: String,
    pub data_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Relationship {
    pub name: String,
    pub left_table: String,
    pub right_table: String,
    pub join_type: String,
    pub join_condition: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomQuery {
    pub name: String,
    pub sql: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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

        // Validate that each table has at least one dimension, time_dimension, or fact
        if table.dimensions.is_empty() && table.time_dimensions.is_empty() && table.facts.is_empty() {
            return Err(ValidationError {
                message: format!(
                    "Table '{}' must have at least one dimension, time_dimension, or fact",
                    table.name
                ),
                is_yaml_error: false,
            });
        }
    }

    Ok(model)
}

/// Format a validation error as a string
pub fn format_error(error: &ValidationError) -> String {
    let mut output = String::new();
    
    output.push_str(&"═".repeat(80));
    output.push('\n');
    output.push_str("  VALIDATION ERROR\n");
    output.push_str(&"═".repeat(80));
    output.push_str("\n\n");
    output.push_str(&format!("✗ {}\n", error.message));
    output.push('\n');
    
    if error.is_yaml_error {
        output.push_str("TIP:\n");
        output.push_str("  Check the YAML syntax at the indicated line and column.\n");
        output.push_str("  Common issues include:\n");
        output.push_str("    • Incorrect indentation (use spaces, not tabs)\n");
        output.push_str("    • Missing colons after keys\n");
        output.push_str("    • Unquoted strings containing special characters\n");
        output.push_str("    • Missing required fields\n");
        output.push('\n');
    }
    
    output.push_str(&"═".repeat(80));
    
    output
}

/// Format a successful validation result as a string
pub fn format_success(model: &SemanticModel) -> String {
    let mut output = String::new();
    
    output.push_str(&"═".repeat(80));
    output.push('\n');
    output.push_str("  SEMANTIC MODEL VALIDATION SUMMARY\n");
    output.push_str(&"═".repeat(80));
    output.push_str("\n\n");

    output.push_str(&format!("Name: {}\n", model.name));
    output.push_str(&format!("Description: {}\n", model.description));
    output.push_str("\n");

    output.push_str("TABLES\n");
    output.push_str(&"─".repeat(80));
    output.push('\n');
    
    for table in &model.tables {
        output.push_str(&format!("  • {}\n", table.name));
        output.push_str(&format!(
            "    Location: {}.{}.{}\n",
            table.base_table.database,
            table.base_table.schema,
            table.base_table.table
        ));
        output.push_str(&format!("    Dimensions: {}\n", table.dimensions.len()));
        output.push_str(&format!("    Time Dimensions: {}\n", table.time_dimensions.len()));
        output.push_str(&format!("    Facts: {}\n", table.facts.len()));
        output.push('\n');
    }

    output.push_str("RELATIONSHIPS\n");
    output.push_str(&"─".repeat(80));
    output.push('\n');
    if model.relationships.is_empty() {
        output.push_str("  No relationships defined\n");
    } else {
        for rel in &model.relationships {
            output.push_str(&format!("  • {}\n", rel.name));
            output.push_str(&format!(
                "    {} {} → {}\n",
                rel.join_type,
                rel.left_table,
                rel.right_table
            ));
        }
    }
    output.push('\n');

    output.push_str("CUSTOM QUERIES\n");
    output.push_str(&"─".repeat(80));
    output.push('\n');
    if model.custom_queries.is_empty() {
        output.push_str("  No custom queries defined\n");
    } else {
        for query in &model.custom_queries {
            output.push_str(&format!("  • {}\n", query.name));
            if let Some(desc) = &query.description {
                output.push_str(&format!("    {}\n", desc));
            }
        }
    }
    output.push('\n');

    output.push_str(&"═".repeat(80));
    output.push('\n');
    output.push_str("✓ Validation successful!\n");
    output.push_str(&"═".repeat(80));
    
    output
}
