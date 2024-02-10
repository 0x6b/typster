use std::{collections::HashMap, path::Path};

use lopdf::{Dictionary, Document, Object};
use serde::{Deserialize, Serialize};
use xmp_toolkit::{xmp_ns, OpenFileOptions, XmpDateTime, XmpFile, XmpMeta, XmpValue};

/// PDF, dublin core, and xmp metadata for a document.
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

/// Update the metadata of a PDF file.
///
/// # Arguments
///
/// - `path` - Path to the PDF file.
/// - `metadata` - Metadata to set.
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::remove_file, path::PathBuf, process::Command};

    use crate::{compile, update_metadata, CompileParams, PdfMetadata};

    #[test]
    fn test_update_metadata() -> Result<(), Box<dyn std::error::Error>> {
        let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-test-update-metadata.pdf");
        let params = CompileParams {
            input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("examples")
                .join("sample.typ"),
            output: output.clone(),
            font_paths: vec![],
            ppi: None,
        };
        assert!(compile(&params).is_ok());

        let mut custom_properties = HashMap::new();
        custom_properties.insert("robots".to_string(), "noindex".to_string());
        custom_properties.insert("custom".to_string(), "properties".to_string());

        let metadata = PdfMetadata {
            title: "Title (typster)".to_string(),
            author: "Author (typster)".to_string(),
            application: "Application (typster)".to_string(),
            subject: "Subject (typster)".to_string(),
            copyright_status: true,
            copyright_notice: "Copyright notice (typster)".to_string(),
            keywords: vec!["typster".to_string(), "rust".to_string(), "pdf".to_string()],
            language: "en".to_string(),
            custom_properties,
        };

        assert!(update_metadata(&output, &metadata).is_ok());
        assert!(&output.exists());
        assert!(output.metadata()?.len() > 0);

        let out = String::from_utf8(Command::new("exiftool").arg(&output).output()?.stdout)?;
        let props = out
            .split('\n')
            .map(|line| line.split(':'))
            .filter_map(|mut line| {
                let key = line.next().unwrap_or_default().trim();
                let value = line.next().unwrap_or_default().trim();
                if !key.is_empty() {
                    Some((key, value))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(props.get("Title"), Some(&"Title (typster)"));
        assert_eq!(props.get("Author"), Some(&"Author (typster)"));
        assert_eq!(props.get("Creator"), Some(&"Application (typster)"));
        assert_eq!(props.get("Producer"), Some(&"Application (typster)"));
        assert_eq!(props.get("Creator Tool"), Some(&"Application (typster)"));
        assert_eq!(props.get("Subject"), Some(&"Subject (typster)"));
        assert_eq!(props.get("Marked"), Some(&"True"));
        assert_eq!(props.get("Rights"), Some(&"Copyright notice (typster)"));
        assert_eq!(props.get("Keywords"), Some(&"typster, rust, pdf"));
        assert_eq!(props.get("Language"), Some(&"en"));
        assert_eq!(props.get("Robots"), Some(&"noindex"));
        assert_eq!(props.get("Custom"), Some(&"properties"));

        remove_file(&output)?;

        Ok(())
    }
}
