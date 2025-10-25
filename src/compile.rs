use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use ecow::eco_format;
use typst::{
    diag::{At, SourceResult, Warned},
    foundations::Smart,
    layout::PagedDocument,
};
use typst_pdf::{PdfOptions, PdfStandards};
use typst_syntax::Span;

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

    /// Custom path to local packages, defaults to system-dependent location
    pub package_path: Option<PathBuf>,

    /// Custom path to package cache, defaults to system-dependent location
    pub package_cache_path: Option<PathBuf>,
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
///     package_path: None,
///     package_cache_path: None,
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
pub fn compile(params: &CompileParams) -> Result<Duration, Box<dyn Error>> {
    let world = SystemWorld::new(
        &params.input,
        &params.font_paths,
        params.dict.clone(),
        &params.package_path,
        &params.package_cache_path,
    )
    .map_err(|err| err.to_string())?;
    let start = std::time::Instant::now();

    let Warned { output, warnings } = typst::compile(&world);
    let result = output.and_then(|document| export(&document, params));

    match result {
        Ok(()) => Ok(start.elapsed()),
        Err(errors) => Err(warnings
            .into_iter()
            .chain(errors)
            .map(|diagnostic| {
                format!(
                    "{:?}: {}\n{}",
                    diagnostic.severity,
                    diagnostic.message.clone(),
                    diagnostic
                        .hints
                        .iter()
                        .map(|e| format!("hint: {e}"))
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
            .into()),
    }
}

/// Export into the target format.
fn export(document: &PagedDocument, params: &CompileParams) -> SourceResult<()> {
    match params.output.extension() {
        Some(ext) if ext.eq_ignore_ascii_case("png") => export_image(document, params),
        _ => export_pdf(document, params),
    }
}

/// Export to one or multiple PNGs.
fn export_image(document: &PagedDocument, params: &CompileParams) -> SourceResult<()> {
    let output = &params.output.to_str().unwrap_or_default();
    let can_handle_multiple = output_template::has_indexable_template(output);

    if !can_handle_multiple && document.pages.len() > 1 {
        panic!("{}", "cannot export multiple images without `{{n}}` in output path");
    }

    document.pages.iter().enumerate().for_each(|(i, page)| {
        let storage;
        let path = if can_handle_multiple {
            storage = output_template::format(output, i + 1, document.pages.len());
            Path::new(&storage)
        } else {
            params.output.as_path()
        };
        let pixmap = typst_render::render(page, params.ppi.unwrap_or(144.0) / 72.0);
        let buf = pixmap.encode_png().unwrap();
        fs::write(path, buf).unwrap();
    });

    Ok(())
}

/// Export to a PDF.
fn export_pdf(document: &PagedDocument, params: &CompileParams) -> SourceResult<()> {
    let options = PdfOptions {
        ident: Smart::Auto,
        timestamp: None,
        page_ranges: None,
        standards: PdfStandards::default(),
        tagged: true,
    };
    fs::write(&params.output, typst_pdf::pdf(document, &options)?)
        .map_err(|err| eco_format!("failed to write PDF: {err}"))
        .at(Span::detached())?;
    Ok(())
}

mod output_template {
    const INDEXABLE: [&str; 3] = ["{p}", "{0p}", "{n}"];

    pub fn has_indexable_template(output: &str) -> bool {
        INDEXABLE.iter().any(|template| output.contains(template))
    }

    pub fn format(output: &str, this_page: usize, total_pages: usize) -> String {
        // Find the base 10 width of number `i`
        fn width(i: usize) -> usize {
            1 + i.checked_ilog10().unwrap_or(0) as usize
        }

        let other_templates = ["{t}"];
        INDEXABLE
            .iter()
            .chain(other_templates.iter())
            .fold(output.to_string(), |out, template| {
                let replacement = match *template {
                    "{p}" => format!("{this_page}"),
                    "{0p}" | "{n}" => format!("{:01$}", this_page, width(total_pages)),
                    "{t}" => format!("{total_pages}"),
                    _ => unreachable!("unhandled template placeholder {template}"),
                };
                out.replace(template, replacement.as_str())
            })
    }
}
