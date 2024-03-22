use std::{
    collections::HashMap,
    error::Error,
    fs::{read_to_string, remove_file, File},
    io::copy,
    path::{Path, PathBuf},
    process::Command,
};

use sha2::{Digest, Sha256};
use test_context::{test_context, TestContext};
use typster::{
    compile, format, set_permission, typst_version, update_metadata, CompileParams, FormatParams,
    PdfMetadata, PermissionParams, PrintPermission,
};

struct TypsterTestContext {
    export_pdf: (PathBuf, CompileParams),
    export_png: (PathBuf, CompileParams),
    update_metadata: (PathBuf, CompileParams),
    set_permission: (PathBuf, (PathBuf, CompileParams)),
    format: (String, FormatParams),
}

impl TestContext for TypsterTestContext {
    fn setup() -> TypsterTestContext {
        let path = |n| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join(n);
        let params = |n| {
            (
                path(n),
                CompileParams {
                    input: path("sample.typ"),
                    output: path(n),
                    ..Default::default()
                },
            )
        };

        TypsterTestContext {
            export_pdf: params("export_pdf.pdf"),
            export_png: params("export_png.png"),
            update_metadata: params("update_metadata.pdf"),
            set_permission: (path("set_permission_protected.pdf"), params("set_permission.pdf")),
            format: (
                read_to_string(path("formatted.typ")).unwrap().trim().to_string(),
                FormatParams { input: path("sample.typ"), column: 80 },
            ),
        }
    }

    fn teardown(self) {}
}

#[test_context(TypsterTestContext)]
#[test]
fn test_export_pdf(
    TypsterTestContext { export_pdf: (out, params), .. }: &TypsterTestContext,
) -> Result<(), Box<dyn Error>> {
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(
        calculate_hash(out)?,
        "f9f09e14e1a9906ca327649b94c7958e304f6e66bc1a378abe77c179f3c49cf0"
    );

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_export_png(
    TypsterTestContext { export_png: (out, params), .. }: &TypsterTestContext,
) -> Result<(), Box<dyn Error>> {
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(
        calculate_hash(out)?,
        "6e75034f19b9046f4f304973e6371cfbce2c090c056e521ae3dad7553777fc10"
    );

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_update_metadata(
    TypsterTestContext { update_metadata: (out, params), .. }: &TypsterTestContext,
) -> Result<(), Box<dyn Error>> {
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

    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert!(update_metadata(out, &metadata).is_ok());
    assert!(out.metadata()?.len() > 0);

    let props = get_properties(out)?;
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

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_set_permission(
    TypsterTestContext {
        set_permission: (out_permission, (out, params)), ..
    }: &TypsterTestContext,
) -> Result<(), Box<dyn Error>> {
    assert!(compile(params).is_ok());
    assert!(set_permission(
        out.clone(),
        out_permission.clone(),
        &PermissionParams {
            owner_password: Some("owner".to_string()),
            allow_print: PrintPermission::None,
            ..Default::default()
        },
    )
    .is_ok());
    assert!(out_permission.exists());
    assert!(out_permission.metadata()?.len() > 0);

    let props = get_properties(out_permission)?;
    assert!(props.get("Encryption").is_some());
    assert_eq!(props.get("User Access"), Some(&"Copy, Annotate, Extract".to_string()));

    remove_file(out)?;
    remove_file(out_permission)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_format(
    TypsterTestContext { format: (expected, params), .. }: &TypsterTestContext,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(*expected, format(params)?);

    Ok(())
}

#[test]
fn test_typst_version() -> Result<(), Box<dyn Error>> {
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
