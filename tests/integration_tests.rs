use std::{
    collections::HashMap,
    fs::{read_to_string, remove_file},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, anyhow};
use sha2_hasher::sync::Sha2Hasher;
use test_context::{TestContext, test_context};
use typster::{
    CompileParams, FormatParams, PdfMetadata, PermissionParams, PrintPermission, compile, format,
    set_permission, typst_version, update_metadata,
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
                FormatParams {
                    input: path("sample.typ"),
                    column: 80,
                    tab_spaces: 2,
                },
            ),
        }
    }

    fn teardown(self) {}
}

#[test_context(TypsterTestContext)]
#[test]
fn test_export_pdf(ctx: &TypsterTestContext) -> Result<()> {
    let TypsterTestContext { export_pdf: (out, params), .. } = ctx;
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(out.sha256()?, "14f11fad6909de1f647ca87598071de7a56d201e79bc28d677fc1dc89d7a16fe");

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_export_png(ctx: &TypsterTestContext) -> Result<()> {
    let TypsterTestContext { export_png: (out, params), .. } = ctx;
    assert!(compile(params).is_ok());
    assert!(out.exists());
    assert_eq!(out.sha256()?, "c0a75e2a658bfac879d2f26fe996e3402629d50a68b5a3075286ac567e576bcf");

    remove_file(out)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_update_metadata(ctx: &TypsterTestContext) -> Result<()> {
    let TypsterTestContext { update_metadata: (out, params), .. } = ctx;
    let mut custom_properties = HashMap::new();
    custom_properties.insert("robots".to_string(), "noindex".to_string());
    custom_properties.insert("custom".to_string(), "properties".to_string());

    let metadata = PdfMetadata {
        title: "Title タイトル (typster)".to_string(),
        author: "Author 著者 (typster)".to_string(),
        application: "Application アプリケーション (typster)".to_string(),
        subject: "Subject 題名 (typster)".to_string(),
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
    assert_eq!(props.get("Title"), Some(&"Title タイトル (typster)".to_string()));
    assert_eq!(props.get("Author"), Some(&"Author 著者 (typster)".to_string()));
    assert_eq!(props.get("Creator"), Some(&"Author 著者 (typster)".to_string()));
    assert_eq!(props.get("Producer"), Some(&"Application アプリケーション (typster)".to_string()));
    assert_eq!(
        props.get("Creator Tool"),
        Some(&"Application アプリケーション (typster)".to_string())
    );
    assert_eq!(props.get("Subject"), Some(&"Subject 題名 (typster)".to_string()));
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
fn test_set_permission(ctx: &TypsterTestContext) -> Result<()> {
    let TypsterTestContext {
        set_permission: (out_permission, (out, params)), ..
    } = ctx;
    assert!(compile(params).is_ok());
    assert!(
        set_permission(
            out.clone(),
            out_permission.clone(),
            &PermissionParams {
                owner_password: Some("owner".to_string()),
                allow_print: PrintPermission::None,
                ..Default::default()
            },
        )
        .is_ok()
    );
    assert!(out_permission.exists());
    assert!(out_permission.metadata()?.len() > 0);

    let props = get_properties(out_permission)?;
    assert!(props.contains_key("Encryption"));
    assert_eq!(props.get("User Access"), Some(&"Copy, Annotate, Extract".to_string()));

    remove_file(out)?;
    remove_file(out_permission)?;
    Ok(())
}

#[test_context(TypsterTestContext)]
#[test]
fn test_format(ctx: &TypsterTestContext) -> Result<()> {
    let TypsterTestContext { format: (expected, params), .. } = ctx;
    assert_eq!(*expected, format(params).map_err(|e| anyhow!(e.to_string()))?.trim());

    Ok(())
}

#[test]
fn test_typst_version() -> Result<()> {
    assert_eq!(typst_version(), "0.14.0");

    Ok(())
}

fn get_properties(path: &Path) -> Result<HashMap<String, String>> {
    if Command::new("exiftool").output().is_err() {
        return Err(anyhow!("ExifTool is not installed or not found in PATH"));
    }

    let out = String::from_utf8(Command::new("exiftool").arg(path).output()?.stdout)?;
    let props = out
        .split('\n')
        .map(|line| line.split(':'))
        .filter_map(|mut line| {
            let key = line.next().unwrap_or_default().trim().to_string();
            let value = line.next().unwrap_or_default().trim().to_string();
            if !key.is_empty() { Some((key, value)) } else { None }
        })
        .collect::<HashMap<_, _>>();

    Ok(props)
}
