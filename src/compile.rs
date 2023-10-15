use std::time::Duration;
use std::{fs, path::Path, path::PathBuf};
use typst::{doc::Document, eval::Tracer, geom::Color, World};

use crate::world::SystemWorld;

/// Parameters for a compilation.
#[derive(Debug, Clone)]
pub struct CompileParams {
    /// Path to input Typst file.
    pub input: PathBuf,

    /// Path to output file (PDF, PNG). Output format is determined by extension, and only PNG and
    /// PDF are supported.
    pub output: PathBuf,

    /// Adds additional directories to search for fonts.
    pub font_paths: Vec<PathBuf>,

    /// Configures the project root (for absolute paths).
    pub root: Option<PathBuf>,

    /// The PPI (pixels per inch) to use for PNG export. None means 144.
    pub ppi: Option<f32>,
}

/// Compiles an input file into a supported output format
pub fn compile(params: &CompileParams) -> Result<Duration, Box<dyn std::error::Error>> {
    let world = SystemWorld::new(&params.input, &params.root, &params.font_paths)?;
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

    for (i, frame) in document.pages.iter().enumerate() {
        let path = if numbered {
            storage = string.replace("{n}", &format!("{:0width$}", i + 1));
            Path::new(&storage)
        } else {
            params.output.as_path()
        };
        let pixmap = typst::export::render(frame, params.ppi.unwrap_or(144.0) / 72.0, Color::WHITE);
        pixmap.save_png(path)?;
    }

    Ok(())
}

/// Export to a PDF.
fn export_pdf(
    document: &Document,
    params: &CompileParams,
) -> Result<(), Box<dyn std::error::Error>> {
    let buffer = typst::export::pdf(document);
    fs::write(&params.output, buffer)?;
    Ok(())
}
