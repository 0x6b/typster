use std::path::PathBuf;

fn main() {
    let params = typster::CompileParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        output: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.pdf"),
        font_paths: vec![],
        dict: vec![("input".to_string(), "value".to_string())],
        ppi: None,
        package_path: None,
        package_cache_path: None,
    };

    typster::list_fonts(&params.font_paths)
        .iter()
        .for_each(|(family, fontinfo)| {
            let mut sorted = fontinfo
                .iter()
                .map(|info| {
                    (format!("{:?}", info.variant.style), format!("{:?}", info.variant.weight))
                })
                .collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

            println!("{}:", family);
            sorted
                .iter()
                .for_each(|(style, weight)| println!("  - Style: {style}, Weight: {weight}"));
        });
}
