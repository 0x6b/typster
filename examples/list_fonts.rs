use std::path::PathBuf;

fn main() {
    // equivalent to:
    //     typst compile examples/sample.typ examples/sample.pdf
    let params = typster::CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        output: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        font_paths: vec![],
        inputs: vec![("input".to_string(), "value".to_string())],
        ppi: None,
    };

    typster::list_fonts(&params.font_paths).iter().for_each(|font| {
        println!("{}:", font.name);
        font.variants
            .iter()
            .for_each(|typster::FontVariant { style, weight, stretch }| {
                println!("- Style: {style:?}, Weight: {weight}, Stretch: {stretch}");
            });
    });
}
