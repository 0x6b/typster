use std::path::PathBuf;

use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let params = typster::CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        output: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        font_paths: vec!["assets".into()],
        ppi: None,
    };

    rt.block_on(async {
        if let Err(error) = typster::watch(&params, true).await {
            eprintln!("Server error: {}", error)
        }
    });
}
