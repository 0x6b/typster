use std::{
    env::var,
    error::Error,
    fs::{File, read_to_string},
    io::Write,
    path::Path,
};

use serde::Deserialize;
use toml::from_str;

#[derive(Deserialize)]
struct ProjectMetadata {
    package: Package,
    dependencies: Dependencies,
}

#[derive(Deserialize)]
struct Package {
    #[serde(rename = "version")]
    typwriter_version: String,
}

#[derive(Deserialize)]
struct Dependencies {
    typst: Typst,
}

#[derive(Deserialize)]
pub struct Typst {
    #[serde(rename = "version")]
    pub typst_version: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = File::create(Path::new(&var("OUT_DIR")?).join("version.rs"))?;
    let ProjectMetadata {
        package: Package { typwriter_version },
        dependencies: Dependencies { typst: Typst { typst_version } },
    } = from_str(&read_to_string("Cargo.toml")?)?;

    // Write the version related functions to a file, which is used in `src/version.rs`, so that it
    // can be included in the binary.
    //
    // See https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates
    write!(
        f,
        r#"/// Returns the version of the library.
///
/// # Example
///
/// ```rust
/// println!("Typwriter version: {{}}", typwriter::version());
/// ```
pub fn version() -> &'static str {{ "{typwriter_version}" }}

/// Returns the Typst version the library was compiled with.
///
/// # Example
///
/// ```rust
/// println!("Typst version: {{}}", typwriter::typst_version());
/// ```
pub fn typst_version() -> &'static str {{ "{typst_version}" }}
"#,
    )
    .map_err(|e| {
        format!(
            "Couldn't write version to {}: {e}",
            Path::new(&var("OUT_DIR").unwrap()).join("version.rs").display(),
        )
        .into()
    })
}
