use clap::Parser as ArgParser;
use log::debug;
use walkdir::WalkDir;

use jib::{lexer::Lexer, parser::parse};

/// A Jib to Javascript compiler.
///
/// Set `RUST_LOG=debug` to enable debug logging.
#[derive(ArgParser, Debug)]
#[command(version)]
pub struct Args {
    /// The source directory.
    #[arg(index = 1, default_value_t = String::from("./"))]
    pub directory: String,
}

fn main() -> Result<(), String> {
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
        let ast_root = parse(&mut lexer).map_err(|(line_number, message)| match line_number {
            Some(line_number) => format!("[{}:{}] {}", filepath.display(), line_number, message),
            None => format!("[{}] {}", filepath.display(), message),
        })?;
        debug!("{:?}", ast_root);
    }
    Ok(())
}
