use std::{error::Error, fmt::Display, path::PathBuf};

use qpdf::{EncryptionParams, EncryptionParamsR6};
use serde::{Deserialize, Serialize};

/// Parameters for PDF permission.
///
/// See also [`set_permission()`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionParams {
    /// User password, which is required to open the document. Set to [`None`] to allow anyone to
    /// open.
    pub user_password: Option<String>,

    /// Owner password, which is required to change permissions. Set to [`None`] to allow anyone to
    /// change.
    pub owner_password: Option<String>,

    /// Allow content copying for accessibility.
    pub allow_accessibility: bool,

    /// Allow page extraction.
    pub allow_extract: bool,

    /// Allow document assembly.
    pub allow_assemble: bool,

    /// Allow commenting and form filling.
    pub allow_annotate_and_form: bool,

    /// Allow form field fill-in or signing.
    pub allow_form_filling: bool,

    /// Allow other modifications.
    pub allow_modify_other: bool,

    /// Allow printing.
    pub allow_print: PrintPermission,

    /// Encrypt metadata.
    pub encrypt_metadata: bool,
}

/// PDF print permission for [`PermissionParams`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrintPermission {
    /// Allow printing in high resolution.
    Full,
    /// Allow printing only in low resolution.
    Low,
    /// Disallow printing.
    None,
}

impl Display for PrintPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintPermission::Full => write!(f, "full"),
            PrintPermission::Low => write!(f, "low"),
            PrintPermission::None => write!(f, "none"),
        }
    }
}

impl From<&PrintPermission> for qpdf::writer::PrintPermission {
    fn from(permission: &PrintPermission) -> qpdf::writer::PrintPermission {
        match permission {
            PrintPermission::Full => qpdf::PrintPermission::Full,
            PrintPermission::Low => qpdf::PrintPermission::Low,
            PrintPermission::None => qpdf::PrintPermission::None,
        }
    }
}

impl From<String> for PrintPermission {
    fn from(permission: String) -> PrintPermission {
        match permission.to_lowercase().as_str() {
            "full" => PrintPermission::Full,
            "low" => PrintPermission::Low,
            _ => PrintPermission::None,
        }
    }
}

impl From<&PermissionParams> for EncryptionParams {
    fn from(params: &PermissionParams) -> EncryptionParams {
        EncryptionParams::R6(EncryptionParamsR6 {
            user_password: params.user_password.clone().unwrap_or_default(),
            owner_password: params.owner_password.clone().unwrap_or_default(),
            allow_accessibility: params.allow_accessibility,
            allow_extract: params.allow_extract,
            allow_assemble: params.allow_assemble,
            allow_annotate_and_form: params.allow_annotate_and_form,
            allow_form_filling: params.allow_form_filling,
            allow_modify_other: params.allow_modify_other,
            allow_print: (&params.allow_print).into(),
            encrypt_metadata: params.encrypt_metadata,
        })
    }
}

impl Default for PermissionParams {
    fn default() -> Self {
        Self {
            user_password: None,
            owner_password: None,
            allow_accessibility: true,
            allow_extract: true,
            allow_assemble: false,
            allow_annotate_and_form: true,
            allow_form_filling: false,
            allow_modify_other: false,
            allow_print: PrintPermission::Full,
            encrypt_metadata: true,
        }
    }
}

/// Sets permission of a PDF file.
///
/// Note that in-place update is not possible, so the output file must be different from the input
/// file. The only supported encryption algorithm is AES-256 (PDF 1.7, or Acrobat 9).
///
/// # Arguments
///
/// - `input` - Path to the input PDF file.
/// - `output` - Path to the output PDF file.
/// - `params` - [`PermissionParams`] to set.
///
/// # Example
///
/// Following is an example of how to use the `set_permission` function:
///
/// ```rust
/// let output = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample.pdf");
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
/// // Then set permission
/// typster::set_permission(
///     output,
///     std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
///         .join("examples")
///         .join("sample-protected.pdf"),
///     &typster::PermissionParams {
///         owner_password: Some("owner".to_string()),
///         allow_print: typster::PrintPermission::None,
///         ..Default::default()
///     },
/// ).unwrap();
/// ```
pub fn set_permission(
    input: PathBuf,
    output: PathBuf,
    params: &PermissionParams,
) -> Result<(), Box<dyn Error>> {
    // Should be canonicalized before equality check, but output is not created yet.
    if input == output {
        return Err("in-place update is not possible".into());
    }
    qpdf::QPdf::read(input)
        .unwrap()
        .writer()
        .encryption_params(params.into())
        .write(output)
        .map_err(|e| e.into())
}
