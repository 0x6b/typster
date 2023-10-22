use std::{
    cell::OnceCell,
    env,
    fs::{self, File},
    path::{Path, PathBuf},
};

use memmap2::Mmap;
use typst::font::{Font, FontBook, FontInfo, FontStyle};
use walkdir::WalkDir;

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

impl FontSearcher {
    /// Create a new, empty system searcher.
    pub fn new() -> Self {
        Self { book: FontBook::new(), fonts: vec![] }
    }

    /// Search everything that is available.
    pub fn search(&mut self, font_paths: &[PathBuf]) {
        for path in font_paths {
            self.search_dir(path)
        }

        self.search_system();

        self.add_embedded();
    }

    /// Add fonts that are embedded in the binary.
    fn add_embedded(&mut self) {
        let mut process = |bytes: &'static [u8]| {
            let buffer = typst::eval::Bytes::from_static(bytes);
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
    }

    /// Search for fonts in the linux system font directories.
    fn search_system(&mut self) {
        if cfg!(target_os = "macos") {
            self.search_dir("/Library/Fonts");
            self.search_dir("/Network/Library/Fonts");
            self.search_dir("/System/Library/Fonts");
        } else if cfg!(unix) {
            self.search_dir("/usr/share/fonts");
            self.search_dir("/usr/local/share/fonts");
        } else if cfg!(windows) {
            self.search_dir(
                env::var_os("WINDIR")
                    .map(PathBuf::from)
                    .unwrap_or_else(|| "C:\\Windows".into())
                    .join("Fonts"),
            );

            if let Some(roaming) = dirs::config_dir() {
                self.search_dir(roaming.join("Microsoft\\Windows\\Fonts"));
            }

            if let Some(local) = dirs::cache_dir() {
                self.search_dir(local.join("Microsoft\\Windows\\Fonts"));
            }
        }

        if let Some(dir) = dirs::font_dir() {
            self.search_dir(dir);
        }
    }

    /// Search for all fonts in a directory recursively.
    fn search_dir(&mut self, path: impl AsRef<Path>) {
        for entry in WalkDir::new(path)
            .follow_links(true)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if matches!(
                path.extension().and_then(|s| s.to_str()),
                Some("ttf" | "otf" | "TTF" | "OTF" | "ttc" | "otc" | "TTC" | "OTC"),
            ) {
                self.search_file(path);
            }
        }
    }

    /// Index the fonts in the file at the given path.
    fn search_file(&mut self, path: &Path) {
        if let Ok(file) = File::open(path) {
            if let Ok(mmap) = unsafe { Mmap::map(&file) } {
                for (i, info) in FontInfo::iter(&mmap).enumerate() {
                    self.book.push(info);
                    self.fonts.push(FontSlot {
                        path: path.into(),
                        index: i as u32,
                        font: OnceCell::new(),
                    });
                }
            }
        }
    }
}

/// Information about a font. Simply a wrapper around `typst::font::book::FontInfo`.
#[derive(Debug)]
pub struct FontInformation {
    /// The name of the font.
    pub name: String,
    /// The variants of the font.
    pub variants: Vec<FontVariant>,
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

impl From<&FontInfo> for FontVariant {
    fn from(info: &FontInfo) -> Self {
        Self {
            style: info.variant.style,
            weight: format!("{:?}", info.variant.weight),
            stretch: format!("{:?}", info.variant.stretch),
        }
    }
}

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
