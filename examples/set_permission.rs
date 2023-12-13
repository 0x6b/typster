use std::{error::Error, path::PathBuf};

use typster::{PermissionParams, PrintPermission};

fn main() -> Result<(), Box<dyn Error>> {
    typster::set_permission(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample-protected.pdf"),
        &PermissionParams {
            owner_password: "owner".to_string(),
            allow_print: PrintPermission::None,
            ..Default::default()
        },
    )
}
