use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, remove_file, File},
    io::copy,
    path::{Path, PathBuf},
    process::Command,
};

use sha2::{Digest, Sha256};
use typster::{
    compile, format, set_permission, typst_version, update_metadata, CompileParams, FormatParams,
    PdfMetadata, PermissionParams, PrintPermission,
};

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let input = PathBuf::from("sample.typ");
    let output = PathBuf::from("sample-test-export-pdf.pdf");
    let params = CompileParams {
        input: input.clone(),
        output: output.clone(),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };

    Ok(())
}

#[test]
fn test_export_pdf() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sample-test-export-pdf.pdf");
    let params = CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sample.typ"),
        output: output.clone(),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };

    compile(&params)?;
    assert!(compile(&params).is_ok());
    assert!(&output.exists());
    assert_eq!(
        calculate_hash(&output)?,
        "f9f09e14e1a9906ca327649b94c7958e304f6e66bc1a378abe77c179f3c49cf0"
    );

    remove_file(&output)?;

    Ok(())
}

#[test]
fn test_export_png() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sample-test-export-png.png");
    let params = CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sample.typ"),
        output: output.clone(),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };

    assert!(compile(&params).is_ok());
    assert!(&output.exists());
    assert_eq!(
        calculate_hash(&output)?,
        "6e75034f19b9046f4f304973e6371cfbce2c090c056e521ae3dad7553777fc10"
    );

    remove_file(&output)?;

    Ok(())
}

#[test]
fn test_format() -> Result<(), Box<dyn Error>> {
    let params = FormatParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sample.typ"),

        column: 80,
    };
    let formatted = read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("formatted.typ"),
    )?
    .trim()
    .to_string();
    assert_eq!(format(&params)?, formatted);

    Ok(())
}

#[test]
fn test_update_metadata() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sample-test-update-metadata.pdf");
    let params = CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sample.typ"),
        output: output.clone(),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
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

    let props = get_properties(&output)?;
    assert_eq!(props.get("Title"), Some(&"Title (typster)".to_string()));
    assert_eq!(props.get("Author"), Some(&"Author (typster)".to_string()));
    assert_eq!(props.get("Creator"), Some(&"Application (typster)".to_string()));
    assert_eq!(props.get("Producer"), Some(&"Application (typster)".to_string()));
    assert_eq!(props.get("Creator Tool"), Some(&"Application (typster)".to_string()));
    assert_eq!(props.get("Subject"), Some(&"Subject (typster)".to_string()));
    assert_eq!(props.get("Marked"), Some(&"True".to_string()));
    assert_eq!(props.get("Rights"), Some(&"Copyright notice (typster)".to_string()));
    assert_eq!(props.get("Keywords"), Some(&"typster, rust, pdf".to_string()));
    assert_eq!(props.get("Language"), Some(&"en".to_string()));
    assert_eq!(props.get("Robots"), Some(&"noindex".to_string()));
    assert_eq!(props.get("Custom"), Some(&"properties".to_string()));

    remove_file(&output)?;

    Ok(())
}

#[test]
fn test_set_permission() -> Result<(), Box<dyn Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sample-test-set-permission.pdf");
    let output_protected = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("sample-test-set-permission-protected.pdf");
    let params = CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("sample.typ"),
        output: output.clone(),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };
    assert!(compile(&params).is_ok());

    assert!(set_permission(
        output.clone(),
        output_protected.clone(),
        &PermissionParams {
            owner_password: Some("owner".to_string()),
            allow_print: PrintPermission::None,
            ..Default::default()
        },
    )
    .is_ok());
    assert!(&output_protected.exists());
    assert!(output_protected.metadata()?.len() > 0);

    let props = get_properties(&output_protected)?;
    assert!(props.get("Encryption").is_some());
    assert_eq!(props.get("User Access"), Some(&"Copy, Annotate, Extract".to_string()));

    // since set_permission embeds time, we can't compare the file hash

    remove_file(&output)?;
    remove_file(&output_protected)?;

    Ok(())
}

#[test]
fn test_version() -> Result<(), Box<dyn Error>> {
    assert_eq!(typst_version(), "0.11.0");
    Ok(())
}

fn calculate_hash(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn get_properties(path: &Path) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let out = String::from_utf8(Command::new("exiftool").arg(path).output()?.stdout)?;
    let props = out
        .split('\n')
        .map(|line| line.split(':'))
        .filter_map(|mut line| {
            let key = line.next().unwrap_or_default().trim().to_string();
            let value = line.next().unwrap_or_default().trim().to_string();
            if !key.is_empty() {
                Some((key, value))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    Ok(props)
}

fn main() {}
