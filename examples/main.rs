use std::path::PathBuf;

fn main() {
    // equivalent to:
    // typst compile --font-path assets/ examples/sample.typ examples/sample.pdf
    let params = typster::CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        output: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        font_paths: vec![],
        root: None,
        ppi: None,
    };
    match typster::compile(&params) {
        Ok(duration) => println!("Compilation succeeded in {duration:?}"),
        Err(why) => eprintln!("{why}"),
    }
}
