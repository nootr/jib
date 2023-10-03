use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
pub enum TokenType {
    #[default]
    Unknown,
}

#[derive(Debug, Default)]
pub struct Token {
    token_type: TokenType,
    filepath: String,
    line_number: usize,
    value: String,
}

fn tokenize_line(line: &str, line_number: usize, filepath: String) -> Vec<Token> {
    vec![Token {
        value: line.to_string(),
        filepath,
        line_number,
        ..Default::default()
    }]
}

pub fn generate_tokens(filepath: &Path) -> impl Iterator<Item = Token> + '_ {
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
