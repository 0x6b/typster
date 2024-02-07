// See `build.rs` for how this file is generated.
include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::typst_version;

    #[test]
    fn test_version() -> Result<(), Box<dyn Error>> {
        assert_eq!(typst_version(), "0.10.0");
        Ok(())
    }
}
