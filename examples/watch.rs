use std::path::PathBuf;

use tokio::runtime::Runtime;
use typster::FittingType;

fn main() {
    tracing_subscriber::fmt::init();
    let rt = Runtime::new().unwrap();
    let params = typster::CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        output: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        font_paths: vec!["assets".into()],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };

    rt.block_on(async {
        if let Err(error) =
            typster::watch(&params, true, Some("Google Chrome.app"), Some(FittingType::Width)).await
        {
            eprintln!("Server error: {}", error)
        }
    });
}
