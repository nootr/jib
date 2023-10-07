#![allow(missing_docs)]
use clap::Parser;

/// A Jib to Javascript compiler.
///
/// Set `RUST_LOG=debug` to enable debug logging.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// The source directory.
    #[arg(index = 1, default_value_t = String::from("./"))]
    pub directory: String,
}

pub fn get_args() -> Args {
    Args::parse()
}
