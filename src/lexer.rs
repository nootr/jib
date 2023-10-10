//! Lexer module for Jib files.

use log::debug;
use regex::Regex;
use std::fs;
use std::path::Path;

/// The [Token] type.
#[allow(missing_docs)]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub enum TokenType {
    #[default]
    Unknown,

    BracketClose,
    BracketOpen,
    Comma,
    Comment,
    CurlyBracketClose,
    CurlyBracketOpen,
    Equal,
    Keyword,
    Minus,
    Newline,
    Period,
    Pipe,
    Plus,
    SemiColon,
    StringLiteral,
    TagScriptEnd,
    TagScriptStart,
    TagStyleEnd,
    TagStyleStart,
    TagTemplateEnd,
    TagTemplateStart,
    Text,
    Whitespace,
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
/// let mut lexer = Lexer::new().load_source("<template>Hello</template>".to_string());
/// for token in &mut lexer {
///     // Do something useful with the token
/// };
/// #
/// # let mut lexer = Lexer::new().load_source("\n\n".to_string());
/// # assert_eq!(lexer.last().unwrap().line_number, 3);
///
/// # let mut lexer = Lexer::new().load_source("<script>Hello</script>".to_string());
/// # assert_eq!(lexer.count(), 3);
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
    /// Compiles a set of regexes, so avoid creating a new Lexer for every source file. Instead, use the
    /// new Lexer that [Lexer::load_file()] returns.
    pub fn new() -> Self {
        Self {
            regexes: Some(Self::compile_regexes()),
            ..Default::default()
        }
    }

    fn compile_regexes() -> Vec<(TokenType, Regex)> {
        vec![
            (TokenType::Comment, Regex::new(r"^#\s*([^\n\r]*)").unwrap()),
            (TokenType::Keyword, Regex::new(r"^(enum)\s").unwrap()),
            (TokenType::Keyword, Regex::new(r"^(type)\s").unwrap()),
            (TokenType::Keyword, Regex::new(r"^(fn)\s").unwrap()),
            (
                TokenType::TagScriptStart,
                Regex::new(r"^(<\s*script\s*>)").unwrap(),
            ),
            (
                TokenType::TagScriptEnd,
                Regex::new(r"^(<\/\s*script\s*>)").unwrap(),
            ),
            (
                TokenType::TagStyleStart,
                Regex::new(r"^(<\s*style\s*>)").unwrap(),
            ),
            (
                TokenType::TagStyleEnd,
                Regex::new(r"^(<\/\s*style\s*>)").unwrap(),
            ),
            (
                TokenType::TagTemplateStart,
                Regex::new(r"^(<\s*template\s*>)").unwrap(),
            ),
            (
                TokenType::TagTemplateEnd,
                Regex::new(r"^(<\/\s*template\s*>)").unwrap(),
            ),
            (TokenType::Newline, Regex::new(r"^([\n\r])").unwrap()),
            (TokenType::Whitespace, Regex::new(r"^([\s\t]+)").unwrap()),
            (TokenType::StringLiteral, Regex::new("^\"(.*?)\"").unwrap()),
            (TokenType::Equal, Regex::new(r"^(=)").unwrap()),
            (TokenType::Minus, Regex::new(r"^(-)").unwrap()),
            (TokenType::Plus, Regex::new(r"^(\+)").unwrap()),
            (TokenType::CurlyBracketOpen, Regex::new(r"^(\{)").unwrap()),
            (TokenType::CurlyBracketClose, Regex::new(r"^(\})").unwrap()),
            (TokenType::BracketOpen, Regex::new(r"^(\()").unwrap()),
            (TokenType::BracketClose, Regex::new(r"^(\))").unwrap()),
            (TokenType::SemiColon, Regex::new(r"^(;)").unwrap()),
            (TokenType::Pipe, Regex::new(r"^(\|)").unwrap()),
            (TokenType::Comma, Regex::new(r"^(,)").unwrap()),
            (TokenType::Period, Regex::new(r"^(\.)").unwrap()),
            (TokenType::Text, Regex::new(r"^([\w:][\w\-:]*)").unwrap()),
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
        let token = Token {
            token_type,
            value: value.unwrap_or_default(),
            filepath: self.filepath.clone(),
            line_number: self.line_number,
        };
        debug!("{:?}", token);
        token
    }

    /// Flushes whitespace and newline tokens.
    pub fn flush_whitespace(&mut self) {
        while let Some(token) = self.peek() {
            if token.token_type != TokenType::Whitespace && token.token_type != TokenType::Newline {
                break;
            }
            self.next();
        }
    }

    /// Returns an error when an unexpected token is encountered.
    pub fn expect_token(
        &mut self,
        expected_token_type: TokenType,
    ) -> Result<Token, (Option<usize>, String)> {
        if let Some(token) = self.next() {
            if token.token_type == expected_token_type {
                Ok(token)
            } else {
                Err((
                    Some(token.line_number),
                    format!(
                        "Expected {:?}, but got {:?}",
                        expected_token_type, token.token_type
                    ),
                ))
            }
        } else {
            Err((None, "Unexpected end of file".to_string()))
        }
    }
}

/// A stream of items which allows for peeking.
pub trait Peekable<Item> {
    /// Returns the next item in the stream.
    fn peek(&mut self) -> Option<Item>;
}

impl Peekable<Token> for Lexer<LoadedSource> {
    /// Returns the next token in the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use jib::lexer::{Lexer, Peekable, TokenType};
    ///
    /// let mut lexer = Lexer::new().load_source("<script>foo</script>".to_string());
    ///
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::TagScriptStart);
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::Text);
    /// assert_eq!(lexer.peek().unwrap().token_type, TokenType::TagScriptEnd);
    /// assert_eq!(lexer.peek().unwrap().token_type, TokenType::TagScriptEnd);
    /// assert_eq!(lexer.next().unwrap().token_type, TokenType::TagScriptEnd);
    /// assert!(lexer.next().is_none());
    /// ```
    fn peek(&mut self) -> Option<Token> {
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

        let (token_type, value, length) = self
            .regexes
            .as_ref()
            .expect("should have compiled regexes")
            .iter()
            // Generate regex matches
            .map(|(t, r)| (t, r.captures(left_to_parse)))
            // Filter matches
            .filter(|(_, m)| m.is_some())
            // Unpack matches
            .map(|(t, m)| {
                (
                    t,
                    m.as_ref().unwrap().get(1).unwrap().as_str().to_string(),
                    m.unwrap().get(0).unwrap().len(),
                )
            })
            // Take single match
            .next()
            // Unknown characters are not always errors, we'll let the parser decide what to do
            // with them.
            .unwrap_or((&TokenType::Unknown, left_to_parse[0..1].to_string(), 1));

        self.offset += length;

        if *token_type == TokenType::Newline {
            self.line_number += 1;
        }

        Some(self.create_token(token_type.clone(), Some(value)))
    }
}
