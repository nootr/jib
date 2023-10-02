use clap::Parser;
use log::debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Default)]
enum TokenType {
    #[default]
    Unknown,
}

#[derive(Debug, Default)]
struct Token {
    token_type: TokenType,
    filepath: String,
    line_number: usize,
    value: String,
}

/// A Jib to Javascript compiler.
///
/// Set `RUST_LOG=debug` to enable debug logging.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// The source directory.
    #[arg(index = 1, default_value_t = String::from("./"))]
    directory: String,
}

fn tokenize_line(line: &str, line_number: usize, filepath: String) -> Vec<Token> {
    vec![Token {
        value: line.to_string(),
        filepath,
        line_number,
        ..Default::default()
    }]
}

fn generate_tokens(filepath: &Path) -> impl Iterator<Item = Token> + '_ {
    let file = File::open(filepath).expect("should open file");
    io::BufReader::new(file)
        .lines()
        .map(|maybe_line| maybe_line.unwrap_or_default())
        .enumerate()
        .flat_map(|(n, l)| {
            tokenize_line(
                &l,
                n + 1,
                filepath.to_str().unwrap_or("UNKNOWN").to_string(),
            )
        })
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .map(|e| e.expect("should find a file or directory"))
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().unwrap_or_default() == "jib")
    {
        let filepath = entry.path();
        debug!("Opening file: `{}`", filepath.display());

        for token in generate_tokens(filepath) {
            debug!("{:?}", token);
        }
    }
}
