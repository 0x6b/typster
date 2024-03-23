//! A naive Rust Library that provides a way to work with [Typst](https://typst.app/) document and PDF file programmatically.
//!
//! # Overview
//!
//! You can use this library to:
//!
//! - [compile](compile()) a Typst file to a PDF file
//! - [format](format()) a Typst file
//! - [set permission](set_permission()) for a PDF file
//! - [update metadata](update_metadata()) of a PDF file
//! - [watch](watch()) for changes in the input Typst file along with its dependencies and recompile
//!   it when a change is detected
//!
//! # Supported Typst Version
//!
//! Version [0.11.0](https://github.com/typst/typst/releases/tag/v0.11.0) (March 15, 2024)
//!
//! This crate is for my personal use and Typst/Rust learning purposes; it is not affiliated with the [Typst](https://typst.app/) project.

#[cfg(feature = "compile")]
pub use compile::{compile, CompileParams};
#[cfg(feature = "compile")]
pub use fonts::list_fonts;
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
