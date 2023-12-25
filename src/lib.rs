#[cfg(feature = "compile")]
pub use compile::{compile, CompileParams};
#[cfg(feature = "compile")]
pub use fonts::{export_fonts, list_fonts, FontInformation, FontVariant};
#[cfg(feature = "format")]
pub use format::{format, FormatParams};
#[cfg(feature = "pdf_permission")]
pub use set_permission::{set_permission, PermissionParams, PrintPermission};
#[cfg(feature = "pdf_metadata")]
pub use update_metadata::{update_metadata, PdfMetadata};
pub use version::{typst_version, version};
#[cfg(feature = "watch")]
pub use watch::watch;

#[cfg(feature = "compile")]
mod compile;
#[cfg(feature = "compile")]
mod download;
#[cfg(feature = "compile")]
mod fonts;
#[cfg(feature = "format")]
mod format;
#[cfg(feature = "compile")]
mod package;
#[cfg(feature = "pdf_permission")]
mod set_permission;
#[cfg(feature = "pdf_metadata")]
mod update_metadata;
mod version;
#[cfg(feature = "watch")]
mod watch;
#[cfg(feature = "compile")]
mod world;
