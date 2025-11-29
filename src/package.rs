use std::path::PathBuf;

use download::downloader;
use typst_kit::package::PackageStorage;

use crate::download;

/// Returns a new package storage for the given args.
pub fn storage(
    package_path: &Option<PathBuf>,
    package_cache_path: &Option<PathBuf>,
) -> PackageStorage {
    PackageStorage::new(package_cache_path.clone(), package_path.clone(), downloader())
}
