use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    typster::export_fonts(&PathBuf::from_str("fonts").unwrap());
}
