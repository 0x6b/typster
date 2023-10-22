use std::path::Path;

use lopdf::{Dictionary, Document, Object};
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpFile, XmpMeta, XmpValue};

/// PDF, dublin core, and xmp metadata for a document.
#[derive(Debug, Clone)]
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
    /// - Apple Preview: Content creator
    pub application: String,

    /// Subject of the document.
    /// - Acrobat Reader: Subject _and_ Description
    /// - Apple Preview: (None)
    pub subject: String,

    /// Copyright status. `true` means `Marked`.
    /// - Acrobat Reader: Copyright Status
    /// - Apple Preview: (None)
    pub copyright_status: bool,

    /// Copyright notice.
    /// - Acrobat Reader: Copyright notice
    /// - Apple Preview: (None)
    pub copyright_notice: String,

    /// Keywords, which should be set as an array, but will be concatenated and set as a single
    /// property.
    /// - Acrobat Reader: Keywords
    /// - Apple Preview: (None)
    pub keywords: Vec<String>,

    /// Language (RFC 3066)
    pub language: String,
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
        }
    }
}

pub fn update_metadata(
    path: &Path,
    metadata: &PdfMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = XmpFile::new()?;
    f.open_file(path, OpenFileOptions::default().only_xmp().for_update())?;

    let mut xmp = XmpMeta::new()?;

    let metadata = metadata.clone();

    xmp.set_property(xmp_ns::DC, "title", &XmpValue::from(metadata.title.clone()))?;
    xmp.set_property(xmp_ns::DC, "creator", &XmpValue::from(metadata.author.clone()))?;
    xmp.set_property(xmp_ns::XMP, "CreatorTool", &XmpValue::from(metadata.application.clone()))?;
    xmp.set_property(xmp_ns::DC, "description", &XmpValue::from(metadata.subject))?;
    xmp.set_property_bool(
        xmp_ns::XMP_RIGHTS,
        "Marked",
        &XmpValue::from(metadata.copyright_status),
    )?;
    xmp.set_property(xmp_ns::DC, "rights", &XmpValue::from(metadata.copyright_notice))?;

    // WORKAROUND:
    //
    // xmp.append_array_item(), or xmp.set_array_item always fails with following error:
    //     XmpError {
    //        error_type: BadSerialize, debug_message: "Can't fit into specified packet size"
    //     }
    //
    // So just concatenate keywords and set as a single property.
    //
    // metadata.keywords.into_iter().for_each(|keyword| {
    //     xmp.append_array_item(
    //         xmp_ns::DC,
    //         &XmpValue::from("subject").set_is_ordered(true),
    //         &XmpValue::from(keyword),
    //     )
    //     .unwrap();
    // });
    xmp.set_property(xmp_ns::DC, "subject", &XmpValue::from(metadata.keywords.join(", ")))?;

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
    dict.set("Author", Object::string_literal(metadata.author));
    dict.set("Creator", Object::string_literal(metadata.application));
    let t = doc.add_object(Object::Dictionary(dict));

    doc.trailer.set("Info", t);
    doc.save(path)?;

    Ok(())
}