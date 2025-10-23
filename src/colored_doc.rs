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
