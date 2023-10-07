use regex::Regex;
use std::fs;
use std::path::Path;
use std::slice::Iter;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub enum TokenType {
    #[default]
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
    Unknown,
}

impl TokenType {
    fn regex(&self) -> Regex {
        match self {
            // TODO: find way to only generate regexes once
            TokenType::Comment => Regex::new(r"^#[^\n\r]*").unwrap(),
            TokenType::TagOpen => Regex::new(r"^<").unwrap(),
            TokenType::TagEndOpen => Regex::new(r"^</").unwrap(),
            TokenType::TagClose => Regex::new(r"^>").unwrap(),
            TokenType::TagSingleClose => Regex::new(r"^/>").unwrap(),
            TokenType::EndOfFile => Regex::new(r"^$").unwrap(),
            TokenType::Whitespace => Regex::new(r"^[\s\t]+").unwrap(),
            TokenType::Newline => Regex::new(r"^[\n\r]").unwrap(),
            TokenType::StringLiteral => Regex::new("^\".*?\"").unwrap(),
            TokenType::Equal => Regex::new(r"^=").unwrap(),
            TokenType::Minus => Regex::new(r"^-").unwrap(),
            TokenType::Plus => Regex::new(r"^\+").unwrap(),
            TokenType::CurlyBracketOpen => Regex::new(r"^\{").unwrap(),
            TokenType::CurlyBracketClose => Regex::new(r"^\}").unwrap(),
            TokenType::BracketOpen => Regex::new(r"^\(").unwrap(),
            TokenType::BracketClose => Regex::new(r"^\)").unwrap(),
            TokenType::SemiColon => Regex::new(r"^;").unwrap(),
            TokenType::Pipe => Regex::new(r"^\|").unwrap(),
            TokenType::Comma => Regex::new(r"^,").unwrap(),
            TokenType::Period => Regex::new(r"^\.").unwrap(),
            TokenType::Text => Regex::new(r"^[\w:][\w\-:]*").unwrap(),
            TokenType::Unknown => panic!("should not call regex() for unknown token type"),
        }
    }

    pub fn into_iter() -> Iter<'static, TokenType> {
        static TOKEN_TYPES: [TokenType; 21] = [
            TokenType::Comment,
            TokenType::Newline,
            TokenType::Whitespace,
            TokenType::TagEndOpen,
            TokenType::TagOpen,
            TokenType::TagClose,
            TokenType::TagSingleClose,
            TokenType::StringLiteral,
            TokenType::Equal,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::CurlyBracketOpen,
            TokenType::CurlyBracketClose,
            TokenType::BracketOpen,
            TokenType::BracketClose,
            TokenType::SemiColon,
            TokenType::Pipe,
            TokenType::Comma,
            TokenType::Period,
            TokenType::Text,
            TokenType::EndOfFile,
        ];
        TOKEN_TYPES.iter()
    }
}

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub filepath: String,
    pub line_number: usize,
    pub value: String,
}

#[derive(Debug)]
pub struct Lexer {
    file_content: String,
    filepath: String,
    offset: usize,
    line_number: usize,
}

impl Lexer {
    pub fn new(filepath: &Path) -> Lexer {
        Lexer {
            file_content: fs::read_to_string(filepath).expect("should be able to read file"),
            filepath: filepath
                .to_str()
                .expect("should be able to convert a path to string")
                .to_string(),
            offset: 0,
            line_number: 1,
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

    fn get_token(&mut self) -> Token {
        let left_to_parse = &self.file_content[self.offset..];

        if left_to_parse.is_empty() {
            return self.create_token(TokenType::EndOfFile, None);
        }

        let (token_type, value) = TokenType::into_iter()
            // Generate regex matches
            .map(|t| (t, t.regex().captures(left_to_parse)))
            // Filter matches
            .filter(|(_, m)| m.is_some())
            // Unpack matches
            .map(|(t, m)| (t, m.unwrap().get(0).unwrap().as_str().to_string()))
            // Take single match
            .next()
            .unwrap_or((&TokenType::Unknown, left_to_parse[0..1].to_string()));

        self.offset += value.len();

        if *token_type == TokenType::Newline {
            self.line_number += 1;
        }

        self.create_token(token_type.clone(), Some(value))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.get_token())
    }
}
