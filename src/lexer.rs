//! Lexer module for Jib files.

use regex::Regex;
use std::fs;
use std::path::Path;

/// The [Token] type.
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
}

/// A token containing all the info for parsing, code generation and troubleshooting.
#[derive(Debug, Default, Clone)]
pub struct Token {
    /// The token type.
    pub token_type: TokenType,

    /// The file path of the source code.
    pub filepath: Option<String>,

    /// The line number in the source code.
    pub line_number: usize,

    /// The original string value within the source code.
    pub value: String,
}

/// A [Lexer] state in which the source code has not been loaded yet.
pub struct MissingSource {}

/// A [Lexer] state in which the source code has been loaded.
pub struct LoadedSource {}

/// A [Lexer] state.
pub trait LexerState {}

impl LexerState for MissingSource {}
impl LexerState for LoadedSource {}

/// Turns source code into a stream of [Token]s.
///
/// # Example
///
/// Iterate over a mutable reference of the Lexer to get the tokens.
///
/// ```
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<div>Hello</div>".to_string());
/// for token in &mut lexer {
///     // Do something useful with the token
/// };
/// #
/// # let mut lexer = Lexer::new().load_source("Line 1\nLine 2\nLine 3".to_string());
/// # assert_eq!(lexer.last().unwrap().line_number, 3);
///
/// # let mut lexer = Lexer::new().load_source("<div>Hello</div>".to_string());
/// # assert_eq!(lexer.count(), 7);
/// ```
///
/// Make sure you do not create a new Lexer for each file when tokenizing multiple files, because
/// [Lexer::new()] will compile a new set of regexes everytime it's executed.
///
/// ```
/// use jib::lexer::Lexer;
///
/// # let files = vec![];
/// let lexer = Lexer::new();
/// for file in files {
///     let mut lexer = lexer.load_file(file);
///     for token in &mut lexer {
///         // Do something useful with the token
///     };
/// }
/// ```
#[derive(Debug)]
pub struct Lexer<S: LexerState> {
    source: Option<String>,
    filepath: Option<String>,
    offset: usize,
    line_number: usize,
    regexes: Option<Vec<(TokenType, Regex)>>,
    peeked_token: Option<Token>,
    marker: std::marker::PhantomData<S>,
}

impl Lexer<MissingSource> {
    /// Creates a new [Lexer].
    ///
    /// Compiles a set of regexes, so avoid creating a new Lexer for every source file, but use the
    /// new Lexer that [Lexer::load_file()] returns.
    pub fn new() -> Self {
        Self {
            regexes: Some(Self::compile_regexes()),
            ..Default::default()
        }
    }

    fn compile_regexes() -> Vec<(TokenType, Regex)> {
        vec![
            (TokenType::Comment, Regex::new(r"^#[^\n\r]*").unwrap()),
            (TokenType::TagOpen, Regex::new(r"^<[^/]").unwrap()),
            (TokenType::TagEndOpen, Regex::new(r"^</").unwrap()),
            (TokenType::TagClose, Regex::new(r"^>").unwrap()),
            (TokenType::TagSingleClose, Regex::new(r"^/>").unwrap()),
            (TokenType::Newline, Regex::new(r"^[\n\r]").unwrap()),
            (TokenType::Whitespace, Regex::new(r"^[\s\t]+").unwrap()),
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
        ]
    }
}

impl<S> Lexer<S>
where
    S: LexerState,
{
    /// Loads a source file.
    pub fn load_file(&self, filepath: &Path) -> Lexer<LoadedSource> {
        Lexer {
            source: Some(fs::read_to_string(filepath).expect("should be able to read file")),
            filepath: Some(
                filepath
                    .to_str()
                    .expect("should be able to convert a path to string")
                    .to_string(),
            ),
            regexes: self.regexes.clone(),
            ..Default::default()
        }
    }

    /// Loads source code.
    pub fn load_source(&self, source: String) -> Lexer<LoadedSource> {
        Lexer {
            source: Some(source),
            regexes: self.regexes.clone(),
            ..Default::default()
        }
    }
}

impl<S> Default for Lexer<S>
where
    S: LexerState,
{
    fn default() -> Lexer<S> {
        Self {
            source: None,
            filepath: None,
            offset: 0,
            line_number: 1,
            regexes: None,
            peeked_token: None,
            marker: std::marker::PhantomData,
        }
    }
}

impl Lexer<LoadedSource> {
    fn create_token(&self, token_type: TokenType, value: Option<String>) -> Token {
        Token {
            token_type,
            value: value.unwrap_or_default(),
            filepath: self.filepath.clone(),
            line_number: self.line_number,
        }
    }

    /// Returns the next token in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use jib::lexer::{Lexer, TokenType};
    ///
    /// let mut lexer = Lexer::new().load_source("<div>".to_string());
    ///
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::TagOpen);
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::Text);
    /// assert_eq!(lexer.peek().unwrap().token_type, TokenType::TagClose);
    /// assert_eq!(lexer.peek().unwrap().token_type, TokenType::TagClose);
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::TagClose);
    /// assert!(lexer.next().is_none());
    /// ```
    pub fn peek(&mut self) -> Option<Token> {
        match &self.peeked_token {
            Some(token) => Some(token.clone()),
            None => {
                self.peeked_token = self.next();
                self.peeked_token.clone()
            }
        }
    }
}

impl Iterator for Lexer<LoadedSource> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked_token.is_some() {
            let token = self.peeked_token.clone();
            self.peeked_token = None;
            return token;
        }

        let left_to_parse = &(self
            .source
            .as_ref()
            .expect("should have loaded source code"))[self.offset..];

        if left_to_parse.is_empty() {
            return None;
        }

        let (token_type, value) = self
            .regexes
            .as_ref()
            .expect("should have compiled regexes")
            .iter()
            // Generate regex matches
            .map(|(t, r)| (t, r.captures(left_to_parse)))
            // Filter matches
            .filter(|(_, m)| m.is_some())
            // Unpack matches
            .map(|(t, m)| (t, m.unwrap().get(0).unwrap().as_str().to_string()))
            // Take single match
            .next()
            // Unknown characters are not always errors, we'll let the parser decide what to do
            // with them.
            .unwrap_or((&TokenType::Unknown, left_to_parse[0..1].to_string()));

        self.offset += value.len();

        if *token_type == TokenType::Newline {
            self.line_number += 1;
        }

        Some(self.create_token(token_type.clone(), Some(value)))
    }
}
