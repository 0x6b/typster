use std::path::PathBuf;

fn main() {
    let params = typster::FormatParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        column: 80,
    };

    println!("{}", typster::format(&params).map_or_else(|why| why.to_string(), |s| s));
}
