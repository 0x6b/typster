use std::path::PathBuf;

use typster::PdfMetadata;

fn main() {
    let metadata = PdfMetadata {
        title: "Title (typster)".to_string(),
        author: "Author (typster)".to_string(),
        application: "Application (typster)".to_string(),
        subject: "Subject (typster)".to_string(),
        copyright_status: true,
        copyright_notice: "Copyright notice (typster)".to_string(),
        keywords: vec!["typster".to_string(), "rust".to_string(), "pdf".to_string()],
        language: "en".to_string(),
    };

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("sample.pdf");

    typster::update_metadata(&path, &metadata).unwrap();
}
