use termcolor::{Color, ColorSpec, WriteColor};

/// A document that can be rendered with colors
#[derive(Clone, Debug)]
pub enum ColoredDoc {
    Text(String, Option<ColorSpec>),
    Line,
    Concat(Vec<ColoredDoc>),
}

impl ColoredDoc {
    /// Create plain text
    pub fn text(s: impl Into<String>) -> Self {
        ColoredDoc::Text(s.into(), None)
    }

    /// Create colored text
    pub fn colored_text(s: impl Into<String>, color: ColorSpec) -> Self {
        ColoredDoc::Text(s.into(), Some(color))
    }

    /// Create a line break
    pub fn line() -> Self {
        ColoredDoc::Line
    }

    /// Concatenate multiple documents
    pub fn concat(docs: Vec<ColoredDoc>) -> Self {
        ColoredDoc::Concat(docs)
    }

    /// Append another document to this one
    pub fn append(self, other: ColoredDoc) -> Self {
        match self {
            ColoredDoc::Concat(mut docs) => {
                docs.push(other);
                ColoredDoc::Concat(docs)
            }
            _ => ColoredDoc::Concat(vec![self, other]),
        }
    }

    /// Render to a writer with colors
    pub fn render_colored<W: WriteColor>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            ColoredDoc::Text(s, color_spec) => {
                if let Some(spec) = color_spec {
                    writer.set_color(spec)?;
                    write!(writer, "{}", s)?;
                    writer.reset()?;
                } else {
                    write!(writer, "{}", s)?;
                }
            }
            ColoredDoc::Line => {
                writeln!(writer)?;
            }
            ColoredDoc::Concat(docs) => {
                for doc in docs {
                    doc.render_colored(writer)?;
                }
            }
        }
        Ok(())
    }

    /// Render to a plain string (no colors)
    pub fn render_plain(&self) -> String {
        let mut buffer = Vec::new();
        let mut writer = termcolor::NoColor::new(&mut buffer);
        self.render_colored(&mut writer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

/// Helper to create color specs
pub fn color_spec(color: Color, bold: bool) -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_fg(Some(color));
    if bold {
        spec.set_bold(true);
    }
    spec
}

/// Helper to create a dimmed color spec
pub fn dimmed_spec() -> ColorSpec {
    let mut spec = ColorSpec::new();
    spec.set_dimmed(true);
    spec
}

/// Create a major heading with separator lines (uses = for separator)
pub fn heading(title: impl Into<String>, color: Color) -> ColoredDoc {
    ColoredDoc::concat(vec![
        ColoredDoc::colored_text("=".repeat(80), color_spec(color, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text(format!("  {}", title.into()), color_spec(color, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("=".repeat(80), color_spec(color, true)),
        ColoredDoc::line(),
    ])
}

/// Create a subheading with separator line (uses - for separator)
pub fn subheading(title: impl Into<String>, color: Color) -> ColoredDoc {
    ColoredDoc::concat(vec![
        ColoredDoc::colored_text(title.into(), color_spec(color, true)),
        ColoredDoc::line(),
        ColoredDoc::colored_text("-".repeat(80), color_spec(Color::Black, true)),
        ColoredDoc::line(),
    ])
}

/// Create a separator line
pub fn separator(char: &str, color: Color) -> ColoredDoc {
    ColoredDoc::concat(vec![
        ColoredDoc::colored_text(char.repeat(80), color_spec(color, true)),
        ColoredDoc::line(),
    ])
}

/// Column alignment for table rendering
#[derive(Clone, Debug, Copy)]
pub enum Alignment {
    Left,
    Right,
}

/// A cell in a table
#[derive(Clone, Debug)]
pub struct Cell {
    content: String,
    color_spec: Option<ColorSpec>,
}

impl Cell {
    /// Create a plain text cell
    pub fn text(s: impl Into<String>) -> Self {
        Cell {
            content: s.into(),
            color_spec: None,
        }
    }

    /// Create a colored cell
    pub fn colored(s: impl Into<String>, color_spec: ColorSpec) -> Self {
        Cell {
            content: s.into(),
            color_spec: Some(color_spec),
        }
    }

    /// Get the plain text length (for width calculation)
    fn plain_len(&self) -> usize {
        self.content.len()
    }
}

/// A column in a table - owns its header, alignment, and all cells
#[derive(Clone, Debug)]
pub struct Column {
    header: String,
    alignment: Alignment,
    cells: Vec<Cell>,
}

impl Column {
    /// Create a new column with a header and default left alignment
    pub fn new(header: impl Into<String>) -> Self {
        Column {
            header: header.into(),
            alignment: Alignment::Left,
            cells: Vec::new(),
        }
    }

    /// Create a new column with a header and specified alignment
    pub fn new_aligned(header: impl Into<String>, alignment: Alignment) -> Self {
        Column {
            header: header.into(),
            alignment,
            cells: Vec::new(),
        }
    }

    /// Add a cell to this column
    pub fn add_cell(mut self, cell: Cell) -> Self {
        self.cells.push(cell);
        self
    }

    /// Get the maximum width needed for this column (header or cells)
    fn calculate_width(&self) -> usize {
        let header_width = self.header.len();
        let max_cell_width = self.cells.iter().map(|c| c.plain_len()).max().unwrap_or(0);
        header_width.max(max_cell_width)
    }
}

/// A table builder for rendering tabular data - composed of columns
#[derive(Clone, Debug)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    /// Create a new empty table
    pub fn new() -> Self {
        Table {
            columns: Vec::new(),
        }
    }

    /// Add a column to the table
    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    /// Get the number of rows (based on the first column, or 0 if no columns)
    fn row_count(&self) -> usize {
        self.columns.first().map(|c| c.cells.len()).unwrap_or(0)
    }

    /// Calculate the width needed for each column
    fn calculate_widths(&self) -> Vec<usize> {
        self.columns
            .iter()
            .map(|col| col.calculate_width())
            .collect()
    }

    /// Render the table as a ColoredDoc
    pub fn render(self) -> ColoredDoc {
        if self.columns.is_empty() {
            return ColoredDoc::text("");
        }

        let widths = self.calculate_widths();
        let row_count = self.row_count();
        let mut doc = ColoredDoc::text("");

        // Render header row
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                doc = doc.append(ColoredDoc::text(" | "));
            }
            let padded = match column.alignment {
                Alignment::Left => format!("{:<width$}", column.header, width = widths[i]),
                Alignment::Right => format!("{:>width$}", column.header, width = widths[i]),
            };
            doc = doc.append(ColoredDoc::colored_text(
                padded,
                color_spec(Color::White, true),
            ));
        }
        doc = doc.append(ColoredDoc::line());

        // Render separator line
        for (i, &width) in widths.iter().enumerate() {
            if i > 0 {
                doc = doc.append(ColoredDoc::text("-|-"));
            }
            doc = doc.append(ColoredDoc::text("-".repeat(width)));
        }
        doc = doc.append(ColoredDoc::line());

        // Render data rows
        for row_idx in 0..row_count {
            for (col_idx, column) in self.columns.iter().enumerate() {
                if col_idx > 0 {
                    doc = doc.append(ColoredDoc::text(" | "));
                }

                // Get the cell for this row (or use empty string if column is shorter)
                if let Some(cell) = column.cells.get(row_idx) {
                    let padded = match column.alignment {
                        Alignment::Left => {
                            format!("{:<width$}", cell.content, width = widths[col_idx])
                        }
                        Alignment::Right => {
                            format!("{:>width$}", cell.content, width = widths[col_idx])
                        }
                    };
                    if let Some(ref spec) = cell.color_spec {
                        doc = doc.append(ColoredDoc::colored_text(padded, spec.clone()));
                    } else {
                        doc = doc.append(ColoredDoc::text(padded));
                    }
                } else {
                    // Column is shorter than others, pad with spaces
                    doc = doc.append(ColoredDoc::text(" ".repeat(widths[col_idx])));
                }
            }
            doc = doc.append(ColoredDoc::line());
        }

        doc
    }

    /// Render with a prefix indentation
    pub fn render_indented(self, indent: &str) -> ColoredDoc {
        if self.columns.is_empty() {
            return ColoredDoc::text("");
        }

        let widths = self.calculate_widths();
        let row_count = self.row_count();
        let mut doc = ColoredDoc::text("");

        // Render header row
        doc = doc.append(ColoredDoc::text(indent));
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                doc = doc.append(ColoredDoc::text(" | "));
            }
            let padded = match column.alignment {
                Alignment::Left => format!("{:<width$}", column.header, width = widths[i]),
                Alignment::Right => format!("{:>width$}", column.header, width = widths[i]),
            };
            doc = doc.append(ColoredDoc::colored_text(
                padded,
                color_spec(Color::White, true),
            ));
        }
        doc = doc.append(ColoredDoc::line());

        // Render separator line
        doc = doc.append(ColoredDoc::text(indent));
        for (i, &width) in widths.iter().enumerate() {
            if i > 0 {
                doc = doc.append(ColoredDoc::text("-|-"));
            }
            doc = doc.append(ColoredDoc::text("-".repeat(width)));
        }
        doc = doc.append(ColoredDoc::line());

        // Render data rows
        for row_idx in 0..row_count {
            doc = doc.append(ColoredDoc::text(indent));
            for (col_idx, column) in self.columns.iter().enumerate() {
                if col_idx > 0 {
                    doc = doc.append(ColoredDoc::text(" | "));
                }

                // Get the cell for this row (or use empty string if column is shorter)
                if let Some(cell) = column.cells.get(row_idx) {
                    let padded = match column.alignment {
                        Alignment::Left => {
                            format!("{:<width$}", cell.content, width = widths[col_idx])
                        }
                        Alignment::Right => {
                            format!("{:>width$}", cell.content, width = widths[col_idx])
                        }
                    };
                    if let Some(ref spec) = cell.color_spec {
                        doc = doc.append(ColoredDoc::colored_text(padded, spec.clone()));
                    } else {
                        doc = doc.append(ColoredDoc::text(padded));
                    }
                } else {
                    // Column is shorter than others, pad with spaces
                    doc = doc.append(ColoredDoc::text(" ".repeat(widths[col_idx])));
                }
            }
            doc = doc.append(ColoredDoc::line());
        }

        doc
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
