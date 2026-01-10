use std::path::PathBuf;

fn main() {
    let params = typwriter::FormatParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        column: 80,
        tab_spaces: 2,
    };

    println!("{}", typwriter::format(&params).unwrap_or_else(|why| why.to_string()));
}
