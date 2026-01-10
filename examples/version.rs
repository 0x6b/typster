use typwriter::{typst_version, version};
fn main() {
    println!("Typwriter version: {} (Typst {})", version(), typst_version());
}
