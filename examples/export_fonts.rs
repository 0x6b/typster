use std::{path::PathBuf, str::FromStr};

fn main() {
    typster::export_fonts(&Vec::new(), &PathBuf::from_str("fonts").unwrap())
        .unwrap()
        .iter()
        .for_each(|f| println!("{}", f.to_string_lossy()));
}
