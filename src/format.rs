use std::{fs, path::PathBuf};

use typstfmt_lib::Config;

/// Parameters for a formatting operation.
#[derive(Debug, Clone, Default)]
pub struct FormatParams {
    /// Path to input Typst file.
    pub input: PathBuf,

    /// Path to typstfmt config TOML file.
    pub config: Option<PathBuf>,
}

/// Format a Typst file.
///
/// # Arguments
///
/// - `params` - FormatParams struct.
///
/// # Returns
///
/// String containing the formatted Typst file.
pub fn format(params: &FormatParams) -> Result<String, Box<dyn std::error::Error>> {
    Ok(typstfmt_lib::format(
        &fs::read_to_string(&params.input)?,
        match &params.config {
            None => Config::default(),
            Some(path) => Config::from_toml(&fs::read_to_string(path)?)?,
        },
    ))
}
