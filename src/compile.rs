use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use typst::{eval::Tracer, model::Document, visualize::Color, World};

use crate::world::SystemWorld;

/// Parameters for a compilation.
#[derive(Debug, Clone, Default)]
pub struct CompileParams {
    /// Path to input Typst file.
    pub input: PathBuf,

    /// Path to output file (PDF, PNG). Output format is determined by extension, and only PNG and
    /// PDF are supported.
    pub output: PathBuf,

    /// Adds additional directories to search for fonts.
    pub font_paths: Vec<PathBuf>,

    /// The PPI (pixels per inch) to use for PNG export. None means 144.
    pub ppi: Option<f32>,
}

/// Compiles an input file into a supported output format
///
/// # Arguments
///
/// - `params` - CompileParams struct.
///
/// # Returns
///
/// Result containing the core::time::Duration of the compilation.
pub fn compile(params: &CompileParams) -> Result<Duration, Box<dyn std::error::Error>> {
    let world = SystemWorld::new(&params.input, &params.font_paths)?;
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
        let pixmap = typst_render::render(frame, params.ppi.unwrap_or(144.0) / 72.0, Color::WHITE);
        pixmap.save_png(path)?;
    }

    Ok(())
}

/// Export to a PDF.
fn export_pdf(
    document: &Document,
    params: &CompileParams,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(
        &params.output,
        typst_pdf::pdf(document, Some(&params.input.to_string_lossy()), None),
    )?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use std::{
        error::Error,
        fs::{remove_file, File},
        io::copy,
    };

    use sha2::{Digest, Sha256};

    use super::*;

    #[test]
    fn test_export_pdf() -> Result<(), Box<dyn Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-export-pdf.pdf");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            ppi: None,
        };

        assert!(compile(&params).is_ok());
        assert!(&output.exists());
        assert_eq!(
            calculate_hash(&output)?,
            "33008dcefe127e6ecae0e434dba4360a00b10e4534bb08f3fcfc916d48be1760"
        );

        remove_file(&output)?;

        Ok(())
    }

    #[test]
    fn test_export_png() -> Result<(), Box<dyn Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-export-png.png");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            ppi: None,
        };

        assert!(compile(&params).is_ok());
        assert!(&output.exists());
        assert_eq!(
            calculate_hash(&output)?,
            "7ee50113c5316123da53248d19e0c0683ec86cef0593156552df2bb240bde5c0"
        );

        remove_file(&output)?;

        Ok(())
    }

    pub fn calculate_hash(path: &Path) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }
}
