use std::{
    cell::OnceCell,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use fontdb::{Database, Source};
use typst::{
    foundations::Bytes,
    text::{Font, FontBook, FontInfo, FontStyle},
};

/// Searches for fonts.
pub struct FontSearcher {
    /// Metadata about all discovered fonts.
    pub book: FontBook,
    /// Slots that the fonts are loaded into.
    pub fonts: Vec<FontSlot>,
}

/// Holds details about the location of a font and lazily the font itself.
pub struct FontSlot {
    /// The path at which the font can be found on the system.
    path: PathBuf,
    /// The index of the font in its collection. Zero if the path does not point
    /// to a collection.
    index: u32,
    /// The lazily loaded font.
    font: OnceCell<Option<Font>>,
}

/// Information about a font variant. Simply a wrapper around `typst::font::FontVariant`.
#[derive(Debug)]
pub struct FontVariant {
    /// The style of the font (normal / italic / oblique).
    pub style: FontStyle,
    /// How heavy the font is (100 - 900).
    pub weight: String,
    /// How condensed or expanded the font is (0.5 - 2.0).
    pub stretch: String,
}

/// Information about a font. Simply a wrapper around `typst::font::book::FontInfo`.
#[derive(Debug)]
pub struct FontInformation {
    /// The name of the font.
    pub name: String,
    /// The variants of the font.
    pub variants: Vec<FontVariant>,
}

impl FontSearcher {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self { book: FontBook::new(), fonts: vec![] }
    }

    /// Search everything that is available.
    pub fn search(&mut self, font_paths: &[PathBuf]) {
        let mut db = Database::new();

        // Font paths have highest priority.
        for path in font_paths {
            db.load_fonts_dir(path);
        }

        // System fonts have second priority.
        // db.load_system_fonts();

        for face in db.faces() {
            let path = match &face.source {
                Source::File(path) | Source::SharedFile(path, _) => path,
                // We never add binary sources to the database, so there
                // shouln't be any.
                Source::Binary(_) => continue,
            };

            let info = db
                .with_face_data(face.id, FontInfo::new)
                .expect("database must contain this font");

            if let Some(info) = info {
                self.book.push(info);
                self.fonts.push(FontSlot {
                    path: path.clone(),
                    index: face.index,
                    font: OnceCell::new(),
                });
            }
        }

        self.add_embedded();
    }

    /// Add fonts that are embedded in the binary.
    fn add_embedded(&mut self) {
        let mut process = |bytes: &'static [u8]| {
            let buffer = Bytes::from_static(bytes);
            for (i, font) in Font::iter(buffer).enumerate() {
                self.book.push(font.info().clone());
                self.fonts.push(FontSlot {
                    path: PathBuf::new(),
                    index: i as u32,
                    font: OnceCell::from(Some(font)),
                });
            }
        };

        macro_rules! add {
            ($filename:literal) => {
                process(include_bytes!(concat!("../assets/", $filename)));
            };
        }

        // Embed default fonts.
        add!("LinLibertine_R.ttf");
        add!("LinLibertine_RB.ttf");
        add!("LinLibertine_RBI.ttf");
        add!("LinLibertine_RI.ttf");
        add!("NewCMMath-Book.otf");
        add!("NewCMMath-Regular.otf");
        add!("NewCM10-Regular.otf");
        add!("NewCM10-Bold.otf");
        add!("NewCM10-Italic.otf");
        add!("NewCM10-BoldItalic.otf");
        add!("DejaVuSansMono.ttf");
        add!("DejaVuSansMono-Bold.ttf");
        add!("DejaVuSansMono-Oblique.ttf");
        add!("DejaVuSansMono-BoldOblique.ttf");

        #[cfg(feature = "embed_cmu_roman")]
        {
            add!("cmunrm.ttf");
        }
        #[cfg(feature = "embed_ia_writer_duo")]
        {
            add!("iAWriterDuoS-Bold.ttf");
            add!("iAWriterDuoS-BoldItalic.ttf");
            add!("iAWriterDuoS-Italic.ttf");
            add!("iAWriterDuoS-Regular.ttf");
        }
        #[cfg(feature = "embed_noto_sans_jp")]
        {
            add!("NotoSansJP-Black.ttf");
            add!("NotoSansJP-Bold.ttf");
            add!("NotoSansJP-ExtraBold.ttf");
            add!("NotoSansJP-ExtraLight.ttf");
            add!("NotoSansJP-Light.ttf");
            add!("NotoSansJP-Medium.ttf");
            add!("NotoSansJP-Regular.ttf");
            add!("NotoSansJP-SemiBold.ttf");
            add!("NotoSansJP-Thin.ttf");
        }
        #[cfg(feature = "embed_noto_serif_jp")]
        {
            add!("NotoSerifJP-Black.otf");
            add!("NotoSerifJP-Bold.otf");
            add!("NotoSerifJP-ExtraLight.otf");
            add!("NotoSerifJP-Light.otf");
            add!("NotoSerifJP-Medium.otf");
            add!("NotoSerifJP-Regular.otf");
            add!("NotoSerifJP-SemiBold.otf");
        }
        #[cfg(feature = "embed_recursive")]
        {
            add!("recursive-static-OTFs.otc");
        }
    }
}

impl FontSlot {
    /// Get the font for this slot.
    pub fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                let data = fs::read(&self.path).ok()?.into();
                Font::new(data, self.index)
            })
            .clone()
    }
}

impl From<&FontInfo> for FontVariant {
    fn from(info: &FontInfo) -> Self {
        Self {
            style: info.variant.style,
            weight: format!("{:?}", info.variant.weight),
            stretch: format!("{:?}", info.variant.stretch),
        }
    }
}

/// List all fonts available for the library.
///
/// # Arguments
///
/// - `font_paths` - Paths to additional font directories.
///
/// # Returns
///
/// A list of FontInformation structs.
pub fn list_fonts(font_paths: &[PathBuf]) -> Vec<FontInformation> {
    let mut searcher = FontSearcher::new();
    searcher.search(font_paths);
    searcher
        .book
        .families()
        .map(|(name, infos)| {
            let mut variants = infos.map(FontVariant::from).collect::<Vec<FontVariant>>();

            variants.sort_by(|a, b| {
                a.style
                    .cmp(&b.style)
                    .then(a.weight.cmp(&b.weight))
                    .then(a.stretch.cmp(&b.stretch))
            });

            FontInformation { name: name.to_string(), variants }
        })
        .collect::<_>()
}

/// Export all fonts available for the library. This is sometime useful for debugging, or for
/// running `typst watch` without having to install fonts separately.
///
/// # Arguments
///
/// - `font_paths` - Paths to additional font directories.
/// - `out_path` - Path to output directory.
pub fn export_fonts(font_paths: &[PathBuf], out_path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(out_path)?;
    let mut searcher = FontSearcher::new();
    searcher.search(font_paths);
    searcher.fonts.iter().filter_map(|slot| slot.get()).for_each(|font| {
        fs::write(
            out_path.join(format!(
                "{}-{}-{}.ttf",
                font.info().family.replace(' ', "_"),
                {
                    let weight = font.info().variant.weight.to_number().to_string();
                    match weight.as_str() {
                        "100" => "Thin".to_string(),
                        "200" => "Extralight".to_string(),
                        "300" => "Light".to_string(),
                        "400" => "Regular".to_string(),
                        "450" => "Book".to_string(), // NewCMMath-Book.otf
                        "500" => "Medium".to_string(),
                        "600" => "Semibold".to_string(),
                        "700" => "Bold".to_string(),
                        "800" => "Extrabold".to_string(),
                        "900" => "Black".to_string(),
                        _ => weight.to_string(),
                    }
                },
                match font.info().variant.style {
                    FontStyle::Normal => "Regular",
                    FontStyle::Italic => "Italic",
                    FontStyle::Oblique => "Oblique",
                }
            )),
            font.data(),
        )
        .unwrap();
    });
    Ok(())
}
