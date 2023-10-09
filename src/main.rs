use clap::Parser;
use log::debug;
use walkdir::WalkDir;

use jib::lexer::Lexer;

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

fn main() {
    env_logger::init();
    let args = Args::parse();

    let lexer = Lexer::new();
    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .map(|e| e.expect("should find a file or directory"))
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().unwrap_or_default() == "jib")
    {
        let filepath = entry.path();
        debug!("Opening file: `{}`", filepath.display());
        let mut lexer = lexer.load_file(filepath);

        for token in &mut lexer {
            debug!("{:?}", token);
        }
    }
}
