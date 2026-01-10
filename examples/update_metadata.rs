use std::{collections::HashMap, path::PathBuf};

use typwriter::{PdfMetadata, update_metadata};

fn main() {
    let mut custom_properties = HashMap::new();
    custom_properties.insert("robots".to_string(), "noindex".to_string());
    custom_properties.insert("custom".to_string(), "properties".to_string());

    let metadata = PdfMetadata {
        title: "Title (typwriter)".to_string(),
        author: "Author (typwriter)".to_string(),
        application: "Application (typwriter)".to_string(),
        subject: "Subject (typwriter)".to_string(),
        copyright_status: true,
        copyright_notice: "Copyright notice (typwriter)".to_string(),
        keywords: vec!["typwriter".to_string(), "rust".to_string(), "pdf".to_string()],
        language: "en".to_string(),
        custom_properties,
    };

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("sample.pdf");

    update_metadata(&path, &metadata).unwrap();
}
