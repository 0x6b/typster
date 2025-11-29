use std::{collections::HashMap, path::PathBuf};

use typster::{PdfMetadata, update_metadata};

fn main() {
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

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("sample.pdf");

    update_metadata(&path, &metadata).unwrap();
}
