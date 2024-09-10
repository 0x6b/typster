use std::{
    collections::HashMap,
    fs::{read_to_string, remove_file},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Result};
use sha2_hasher::Sha2Hasher;
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
) -> Result<()> {
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(out.sha256()?, "38c041a1439b5303f0e2acff2f2145294eb80ee9f54d5bf7bd7ea4007034921f");

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_export_png(
    TypsterTestContext { export_png: (out, params), .. }: &TypsterTestContext,
) -> Result<()> {
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(out.sha256()?, "6e75034f19b9046f4f304973e6371cfbce2c090c056e521ae3dad7553777fc10");

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_update_metadata(
    TypsterTestContext { update_metadata: (out, params), .. }: &TypsterTestContext,
) -> Result<()> {
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
) -> Result<()> {
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
) -> Result<()> {
    assert_eq!(*expected, format(params).map_err(|e| anyhow!(e.to_string()))?);

    Ok(())
}

#[test]
fn test_typst_version() -> Result<()> {
    assert_eq!(typst_version(), "0.11.1");

    Ok(())
}

fn get_properties(path: &Path) -> Result<HashMap<String, String>> {
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
