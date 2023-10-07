use std::fs;
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
            line_number: 0,
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

    pub fn get_token(&mut self) -> Token {
        let left_to_parse = &self.file_content[self.offset..];

        // TODO: find value using regexes
        let value = if left_to_parse.len() > 8 {
            left_to_parse[..8].to_string()
        } else {
            left_to_parse.to_string()
        };

        self.offset += value.len();

        if !value.is_empty() {
            self.create_token(TokenType::Text, Some(value))
        } else {
            self.create_token(TokenType::EndOfFile, None)
        }
    }
}
