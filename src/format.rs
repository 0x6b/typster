use std::{fs::read_to_string, path::PathBuf};

use typst_syntax::parse;
use typstyle::{get_no_format_nodes, PrettyPrinter};

/// Parameters for a formatting operation.
#[derive(Debug, Clone, Default)]
pub struct FormatParams {
    /// Path to input Typst file.
    pub input: PathBuf,

    /// The width of the output.
    pub column: usize,
}

/// Formats a [Typst](https://typst.app/) file with [Enter-tainer/typstyle](https://github.com/Enter-tainer/typstyle/).
///
/// # Argument
///
/// - `params` - [`FormatParams`] struct.
///
/// # Returns
///
/// String containing the formatted Typst file.
pub fn format(params: &FormatParams) -> Result<String, Box<dyn std::error::Error>> {
    let root = parse(&read_to_string(&params.input)?);
    let disabled_nodes = get_no_format_nodes(root.clone());
    let markup = root.cast().unwrap();
    let printer = PrettyPrinter::new(disabled_nodes);
    let doc = printer.convert_markup(markup);
    Ok(doc.pretty(params.column).to_string())
}
