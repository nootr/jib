//! Lexer library for Jib files

use regex::Regex;
use std::fs;
use std::path::Path;

/// The token type.
#[allow(missing_docs)]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub enum TokenType {
    #[default]
    Unknown,
    Comment,
    Text,
    TagOpen,
    TagEndOpen,
    TagClose,
    TagSingleClose,
    Newline,
    Whitespace,
    StringLiteral,
    Equal,
    Minus,
    Plus,
    CurlyBracketOpen,
    CurlyBracketClose,
    BracketOpen,
    BracketClose,
    SemiColon,
    Pipe,
    Comma,
    Period,
    EndOfFile,
}

/// A token containing all the info for parsing, code generation and troubleshooting.
#[derive(Debug, Default)]
pub struct Token {
    /// The token type.
    pub token_type: TokenType,

    /// The file path of the source code.
    pub filepath: String,

    /// The line number in the source code.
    pub line_number: usize,

    /// The original string value within the source code.
    pub value: String,
}

/// Turns source code into a stream of tokens.
///
/// Use `Lexer::into_iter()` to iterate over the tokens.
///
/// # Example
///
/// ```
/// use jib::lexer::Lexer;
///
/// let lexer = Lexer::from_source("<div>Hello</div>".to_string(), None);
/// assert_eq!(lexer.into_iter().count(), 7);
/// ```
#[derive(Debug)]
pub struct Lexer {
    file_content: String,
    filepath: String,
    offset: usize,
    line_number: usize,
    regexes: Vec<(TokenType, Regex)>,
}

impl Lexer {
    /// Creates a new lexer.
    pub fn new(filepath: &Path) -> Lexer {
        let file_content = fs::read_to_string(filepath).expect("should be able to read file");
        let filepath = filepath
            .to_str()
            .expect("should be able to convert a path to string")
            .to_string();

        Lexer::from_source(file_content, Some(filepath))
    }

    /// Creates a new lexer from a string.
    pub fn from_source(file_content: String, filepath: Option<String>) -> Lexer {
        Lexer {
            file_content,
            filepath: filepath.unwrap_or_default(),
            offset: 0,
            line_number: 1,
            regexes: vec![
                (TokenType::Comment, Regex::new(r"^#[^\n\r]*").unwrap()),
                (TokenType::TagOpen, Regex::new(r"^<[^/]").unwrap()),
                (TokenType::TagEndOpen, Regex::new(r"^</").unwrap()),
                (TokenType::TagClose, Regex::new(r"^>").unwrap()),
                (TokenType::TagSingleClose, Regex::new(r"^/>").unwrap()),
                (TokenType::EndOfFile, Regex::new(r"^$").unwrap()),
                (TokenType::Whitespace, Regex::new(r"^[\s\t]+").unwrap()),
                (TokenType::Newline, Regex::new(r"^[\n\r]").unwrap()),
                (TokenType::StringLiteral, Regex::new("^\".*?\"").unwrap()),
                (TokenType::Equal, Regex::new(r"^=").unwrap()),
                (TokenType::Minus, Regex::new(r"^-").unwrap()),
                (TokenType::Plus, Regex::new(r"^\+").unwrap()),
                (TokenType::CurlyBracketOpen, Regex::new(r"^\{").unwrap()),
                (TokenType::CurlyBracketClose, Regex::new(r"^\}").unwrap()),
                (TokenType::BracketOpen, Regex::new(r"^\(").unwrap()),
                (TokenType::BracketClose, Regex::new(r"^\)").unwrap()),
                (TokenType::SemiColon, Regex::new(r"^;").unwrap()),
                (TokenType::Pipe, Regex::new(r"^\|").unwrap()),
                (TokenType::Comma, Regex::new(r"^,").unwrap()),
                (TokenType::Period, Regex::new(r"^\.").unwrap()),
                (TokenType::Text, Regex::new(r"^[\w:][\w\-:]*").unwrap()),
            ],
        }
    }

    fn create_token(&self, token_type: TokenType, value: Option<String>) -> Token {
        Token {
            token_type,
            value: value.unwrap_or_default(),
            filepath: self.filepath.clone(),
            line_number: self.line_number,
        }
    }

    fn get_token(&mut self) -> Option<Token> {
        let left_to_parse = &self.file_content[self.offset..];

        if left_to_parse.is_empty() {
            return None;
        }

        let (token_type, value) = self
            .regexes
            .clone()
            .into_iter()
            // Generate regex matches
            .map(|(t, r)| (t, r.captures(left_to_parse)))
            // Filter matches
            .filter(|(_, m)| m.is_some())
            // Unpack matches
            .map(|(t, m)| (t, m.unwrap().get(0).unwrap().as_str().to_string()))
            // Take single match
            .next()
            .unwrap_or((TokenType::Unknown, left_to_parse[0..1].to_string()));

        self.offset += value.len();

        if token_type == TokenType::Newline {
            self.line_number += 1;
        }

        Some(self.create_token(token_type.clone(), Some(value)))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_token()
    }
}
