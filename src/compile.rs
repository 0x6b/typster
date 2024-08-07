use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use typst::{eval::Tracer, foundations::Smart, model::Document, visualize::Color, World};

use crate::world::SystemWorld;

/// Parameters for Typst document compilation.
///
/// See also [`compile()`].
#[derive(Debug, Clone, Default)]
pub struct CompileParams {
    /// Path to the input Typst file.
    pub input: PathBuf,

    /// String key-value pairs visible through `sys.inputs` [dictionary](https://typst.app/docs/reference/foundations/dictionary/) in the `input` document.
    pub dict: Vec<(String, String)>,

    /// Path to the output file (PDF, PNG). Output format is determined by extension, and only PNG
    /// and PDF are supported.
    pub output: PathBuf,

    /// Adds additional directories to search for fonts.
    pub font_paths: Vec<PathBuf>,

    /// The PPI (pixels per inch) to use for PNG export. [`None`] means 144.
    pub ppi: Option<f32>,
}

/// Compiles an input file into a supported output format.
///
/// # Argument
///
/// - `params` - [`CompileParams`] struct.
///
/// # Returns
///
/// Result containing the [`Duration`] of the compilation.
///
/// # Example
///
/// Following is an example of how to use the `compile` function:
///
/// ```rust
/// let params = typster::CompileParams {
///     input: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.typ"),
///     output: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.pdf"),
///     font_paths: vec!["assets".into()],
///     dict: vec![("input".to_string(), "value".to_string())],
///     ppi: None,
/// };
/// match typster::compile(&params) {
///     Ok(duration) => println!("Compilation succeeded in {duration:?}"),
///     Err(why) => eprintln!("{why}"),
/// }
/// ```
///
/// which is equivalent to running:
///
/// ```console
/// $ typst compile examples/sample.typ examples/sample.pdf
/// ```
pub fn compile(params: &CompileParams) -> Result<Duration, Box<dyn std::error::Error>> {
    let world = SystemWorld::new(&params.input, &params.font_paths, params.dict.clone())
        .map_err(|err| err.to_string())?;
    let start = std::time::Instant::now();

    // Ensure that the main file is present.
    world.source(world.main()).map_err(|err| err.to_string())?;

    let mut tracer = Tracer::new();
    match typst::compile(&world, &mut tracer) {
        Ok(document) => {
            export(&document, params)?;
            Ok(start.elapsed())
        }
        Err(why) => Err(format!("Error {why:?}").into()),
    }
}

/// Export into the target format.
fn export(document: &Document, params: &CompileParams) -> Result<(), Box<dyn std::error::Error>> {
    match params.output.extension() {
        Some(ext) if ext.eq_ignore_ascii_case("png") => export_image(document, params),
        _ => export_pdf(document, params),
    }
}

/// Export to one or multiple PNGs.
fn export_image(
    document: &Document,
    params: &CompileParams,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine whether we have a `{n}` numbering.
    let string = &params.output.to_str().unwrap_or_default();
    let numbered = string.contains("{n}");
    if !numbered && document.pages.len() > 1 {
        panic!("{}", "cannot export multiple images without `{{n}}` in output path");
    }

    // Find a number width that accommodates all pages. For instance, the
    // first page should be numbered "001" if there are between 100 and
    // 999 pages.
    let width = 1 + document.pages.len().checked_ilog10().unwrap_or(0) as usize;
    let mut storage;

    for (i, page) in document.pages.iter().enumerate() {
        let path = if numbered {
            storage = string.replace("{n}", &format!("{:0width$}", i + 1));
            Path::new(&storage)
        } else {
            params.output.as_path()
        };
        let pixmap =
            typst_render::render(&page.frame, params.ppi.unwrap_or(144.0) / 72.0, Color::WHITE);
        pixmap.save_png(path)?;
    }

    Ok(())
}

/// Export to a PDF.
fn export_pdf(
    document: &Document,
    params: &CompileParams,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&params.output, typst_pdf::pdf(document, Smart::Auto, None))?;
    Ok(())
}
