use std::{fs::read_to_string, path::PathBuf};

use typst_syntax::parse;
use typstyle_core::{strip_trailing_whitespace, AttrStore, PrettyPrinter, PrinterConfig};

/// Parameters for a formatting operation.
///
/// See also [`format()`].
#[derive(Debug, Clone, Default)]
pub struct FormatParams {
    /// Path to the input Typst file.
    pub input: PathBuf,

    /// The width of the output.
    pub column: usize,
}

/// Formats a Typst file with [Enter-tainer/typstyle](https://github.com/Enter-tainer/typstyle/).
///
/// # Argument
///
/// - `params` - [`FormatParams`] struct.
///
/// # Returns
///
/// String containing the formatted Typst file.
///
/// # Example
///
/// Following is an example of how to use the `format` function:
///
/// ```rust
/// let params = typster::FormatParams {
///     input: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.typ"),
///     column: 80,
/// };
///
/// println!("{}", typster::format(&params).map_or_else(|why| why.to_string(), |s| s));
/// ```
pub fn format(params: &FormatParams) -> Result<String, Box<dyn std::error::Error>> {
    let root = parse(&read_to_string(&params.input)?);
    let config = PrinterConfig { max_width: params.column, ..Default::default() };
    let attr_store = AttrStore::new(&root);
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::new(config, attr_store);
    let doc = printer.convert_markup(markup);
    Ok(strip_trailing_whitespace(&doc.pretty(params.column).to_string()))
}
