use std::{
    env::var,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    dependencies: Dependencies,
}

#[derive(Deserialize)]
struct Dependencies {
    typst: Typst,
}

#[derive(Deserialize)]
pub struct Typst {
    pub tag: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::create(Path::new(&var("OUT_DIR")?).join("version.rs"))?;

    let mut file = File::open("Cargo.toml")?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    let package: Package = toml::from_str(&toml_string)?;

    // Write the version string to a file so it can be included in the binary.
    // This is used by the `version` function in `src/version.rs`. Equivalent to:
    //
    // ```rust
    // pub fn typst_version() -> &'static str { "..." }
    // ```
    //
    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    write!(
        f,
        r#"pub fn typst_version() -> &'static str {{ "{}" }}"#,
        package.dependencies.typst.tag
    )
    .map_err(|e| {
        format!(
            "Couldn't write version to {}: {}",
            Path::new(&var("OUT_DIR").unwrap()).join("version.rs").display(),
            e
        )
        .into()
    })
}
