use std::path::PathBuf;

fn main() {
    // equivalent to:
    //     typstfmt examples/sample.typ
    // with the default configuration:
    //     indent_space = 2
    //     max_line_length = 80
    //     line_wrap = true
    //     experimental_args_breaking_consecutive = false
    let params = typster::FormatParams {
        input: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join("sample.typ"),
        config: None,
    };

    println!("{}", typster::format(&params).map_or_else(|why| why.to_string(), |s| s));
}
