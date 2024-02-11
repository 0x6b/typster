use std::{error::Error, fmt::Display, path::PathBuf};

use qpdf::{EncryptionParams, EncryptionParamsR6};
use serde::{Deserialize, Serialize};

/// Parameters for permission.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionParams {
    /// User password, which is required to open the document. Leave empty to allow anyone to open.
    pub user_password: String,

    /// Owner password, which is required to change permissions. Leave empty to allow anyone to
    /// change.
    pub owner_password: String,

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
            user_password: params.user_password.clone(),
            owner_password: params.owner_password.clone(),
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
            user_password: "".to_string(),
            owner_password: "".to_string(),
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

pub fn set_permission(
    input: PathBuf,
    output: PathBuf,
    params: &PermissionParams,
) -> Result<(), Box<dyn Error>> {
    qpdf::QPdf::read(input)
        .unwrap()
        .writer()
        .encryption_params(params.into())
        .write(output)
        .map_err(|e| e.into())
}
