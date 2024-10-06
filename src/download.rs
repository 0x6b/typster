use typst_kit::download::Downloader;

/// Returns a new downloader.
pub fn downloader() -> Downloader {
    Downloader::new(concat!("typster/", env!("CARGO_PKG_VERSION")))
}
