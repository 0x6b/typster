use std::{error::Error, path::PathBuf};

use qpdf::{EncryptionParams, EncryptionParamsR2};

/// Parameters for permission.
#[derive(Debug, Clone)]
pub struct PermissionParams {
    /// User password, which is required to open the document. Leave empty to allow anyone to open.
    pub user_password: String,

    /// Owner password, which is required to change permissions. Leave empty to allow anyone to
    /// change.
    pub owner_password: String,

    /// Allow extraction of text and images.
    pub allow_extract: bool,

    /// Allow printing.
    pub allow_print: bool,

    /// Allow modification.
    pub allow_modify: bool,

    /// Allow annotation.
    pub allow_annotate: bool,
}

impl From<&PermissionParams> for EncryptionParams {
    fn from(params: &PermissionParams) -> EncryptionParams {
        EncryptionParams::R2(EncryptionParamsR2 {
            user_password: params.user_password.clone(),
            owner_password: params.owner_password.clone(),
            allow_extract: params.allow_extract,
            allow_print: params.allow_print,
            allow_modify: params.allow_modify,
            allow_annotate: params.allow_annotate,
        })
    }
}

impl Default for PermissionParams {
    fn default() -> Self {
        Self {
            user_password: "".to_string(),
            owner_password: "".to_string(),
            allow_extract: true,
            allow_print: true,
            allow_modify: false,
            allow_annotate: true,
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
