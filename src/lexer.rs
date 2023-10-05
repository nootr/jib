use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

#[derive(Debug, Default, Eq, PartialEq)]
pub enum TokenType {
    #[default]
    Text,
    TagOpen,
    TagClose,
    TagSingle,
    EndOfFile,
}

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub filepath: String,
    pub line_number: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct Lexer<S: LexerState> {
    stream: Lines<io::BufReader<File>>,
    filepath: String,
    line_number: usize,
    marker: std::marker::PhantomData<S>,
}

pub enum Initial {}
pub enum ReadingText {}

pub trait LexerState {}

impl LexerState for Initial {}
impl LexerState for ReadingText {}

impl Lexer<Initial> {
    pub fn new(filepath: &Path) -> Self {
        let file = File::open(filepath).expect("should be able to open a file");

        Self {
            stream: io::BufReader::new(file).lines(),
            filepath: filepath
                .to_str()
                .expect("should be able to convert a path to string")
                .to_string(),
            line_number: 0,
            marker: std::marker::PhantomData,
        }
    }

    pub fn get_token(&mut self) -> Result<Token, io::Error> {
        let maybe_value = self.stream.next();
        self.line_number += 1;

        if let Some(value) = maybe_value {
            Ok(Token {
                filepath: self.filepath.clone(),
                line_number: self.line_number,
                value: value?,
                ..Default::default()
            })
        } else {
            Ok(Token {
                token_type: TokenType::EndOfFile,
                filepath: self.filepath.clone(),
                line_number: self.line_number,
                ..Default::default()
            })
        }
    }
}
