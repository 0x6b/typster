mod compile;
mod download;
mod fonts;
mod package;
mod world;

use std::time::Duration;

pub use compile::CompileParams;

pub fn compile(params: &CompileParams) -> Result<Duration, Box<dyn std::error::Error>> {
    compile::compile(params)
}
