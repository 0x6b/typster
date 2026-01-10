use typst_kit::download::Downloader;

/// Returns a new downloader.
pub fn downloader() -> Downloader {
    Downloader::new(concat!("typwriter/", env!("CARGO_PKG_VERSION")))
}
