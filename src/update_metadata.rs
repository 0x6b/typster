use std::{collections::HashMap, path::Path};

use lopdf::{Dictionary, Document, Object};
use serde::{Deserialize, Serialize};
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpDateTime, XmpFile, XmpMeta, XmpValue};

/// PDF, dublin core, and [Extensible Metadata Platform (XMP)](https://www.adobe.com/devnet/xmp.html) metadata for a PDF document. See also [Extensible Metadata Platform (XMP) Specification: Part 1, Data Model, Serialization, and Core Properties](https://github.com/adobe/XMP-Toolkit-SDK/blob/main/docs/XMPSpecificationPart1.pdf) for detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    /// Title of the document.
    /// - Acrobat Reader: Title
    /// - Apple Preview: Title
    pub title: String,

    /// Author of the document.
    /// - Acrobat Reader: Author
    /// - Apple Preview: Author
    pub author: String,

    /// Application.
    /// - Acrobat Reader: Application
    /// - Apple Preview: PDF Producer _and_ Content creator
    pub application: String,

    /// Subject of the document.
    /// - Acrobat Reader: Subject _and_ Description
    /// - Apple Preview: Subject
    pub subject: String,

    /// Copyright status. `true` means `Marked`.
    /// - Acrobat Reader: Copyright Status
    /// - Apple Preview: (None)
    pub copyright_status: bool,

    /// Copyright notice.
    /// - Acrobat Reader: Copyright Notice
    /// - Apple Preview: (None)
    pub copyright_notice: String,

    /// Keywords, which should be set as an array, but will be concatenated and set as a single
    /// property.
    /// - Acrobat Reader: Keywords
    /// - Apple Preview: (None)
    pub keywords: Vec<String>,

    /// Language (RFC 3066)
    pub language: String,

    /// Custom properties.
    /// - Acrobat Reader: Custom properties
    /// - Apple Preview: (None)
    pub custom_properties: HashMap<String, String>,
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            author: "".to_string(),
            application: "typster".to_string(),
            subject: "".to_string(),
            copyright_status: true,
            copyright_notice: "© 2023 Author. All rights reserved.".to_string(),
            keywords: vec![],
            language: "en".to_string(),
            custom_properties: HashMap::new(),
        }
    }
}

/// Updates the metadata of a PDF file.
///
/// Note that:
///
/// - All metadata will be overwritten, not merged.
/// - Both creation and modification date are set automatically to the current date _without_ time
///   information_ which means time is always 0:00 UTC, for some privacy reasons (or my preference.)
///
/// # Arguments
///
/// - `path` - Path to the PDF file.
/// - `metadata` - [`PdfMetadata`] to set.
///
/// # Example
///
/// Following is an example of how to use the `update_metadata` function:
///
/// ```rust
/// let output = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///     .join("examples")
///     .join("sample.pdf");
///
/// // Compile a document first
/// let params = typster::CompileParams {
///     input: std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.typ"),
///     output: output.clone(),
///     font_paths: vec!["assets".into()],
///     dict: vec![("input".to_string(), "value".to_string())],
///     ppi: None,
/// };
/// match typster::compile(&params) {
///     Ok(duration) => println!("Compilation succeeded in {duration:?}"),
///     Err(why) => eprintln!("{why}"),
/// }
///
/// // Then update metadata
/// let mut custom_properties = std::collections::HashMap::new();
/// custom_properties.insert("robots".to_string(), "noindex".to_string());
/// custom_properties.insert("custom".to_string(), "properties".to_string());
///
/// let metadata = typster::PdfMetadata {
///     title: "Title (typster)".to_string(),
///     author: "Author (typster)".to_string(),
///     application: "Application (typster)".to_string(),
///     subject: "Subject (typster)".to_string(),
///     copyright_status: true,
///     copyright_notice: "Copyright notice (typster)".to_string(),
///     keywords: vec!["typster".to_string(), "rust".to_string(), "pdf".to_string()],
///     language: "en".to_string(),
///     custom_properties,
/// };
///
/// typster::update_metadata(&output, &metadata).unwrap();
/// ```
pub fn update_metadata(
    path: &Path,
    metadata: &PdfMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = XmpFile::new()?;
    f.open_file(path, OpenFileOptions::default().only_xmp().for_update())?;

    let mut xmp = XmpMeta::new()?;

    let metadata = metadata.clone();

    xmp.set_property(xmp_ns::DC, "title", &XmpValue::from(metadata.title.clone()))?;
    xmp.set_property(xmp_ns::XMP, "CreatorTool", &XmpValue::from(metadata.application.clone()))?;
    xmp.set_property(xmp_ns::DC, "description", &XmpValue::from(metadata.subject.clone()))?;
    xmp.set_property_bool(
        xmp_ns::XMP_RIGHTS,
        "Marked",
        &XmpValue::from(metadata.copyright_status),
    )?;
    xmp.set_property(xmp_ns::DC, "rights", &XmpValue::from(metadata.copyright_notice))?;
    let mut now = XmpDateTime::current()?;
    now.time = None;
    xmp.set_property_date(xmp_ns::XMP, "CreateDate", &XmpValue::from(now.clone()))?;
    xmp.set_property_date(xmp_ns::XMP, "ModifyDate", &XmpValue::from(now))?;

    // check if xmp can be updated
    if !f.can_put_xmp(&xmp) {
        return Err("cannot update metadata for some reason".into());
    }

    f.put_xmp(&xmp)?;
    f.close();

    let mut doc = Document::load(path)?;
    doc.trailer.remove(b"Info");

    let mut dict = Dictionary::new();
    dict.set("Title", Object::string_literal(metadata.title));
    dict.set("Subject", Object::string_literal(metadata.subject));
    dict.set("Author", Object::string_literal(metadata.author.clone()));
    dict.set("Producer", Object::string_literal(metadata.application.clone()));
    dict.set("Creator", Object::string_literal(metadata.application));
    let now = chrono::Local::now().format("%Y%m%d").to_string();
    dict.set("CreationDate", Object::string_literal(now.clone()));
    dict.set("ModDate", Object::string_literal(now));
    dict.set("Keywords", Object::string_literal(metadata.keywords.join(", ")));
    metadata
        .custom_properties
        .into_iter()
        .for_each(|(k, v)| dict.set(k, Object::string_literal(v)));
    let t = doc.add_object(Object::Dictionary(dict));

    doc.trailer.set("Info", t);
    doc.save(path)?;

    Ok(())
}
