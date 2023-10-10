//! Parser module for Jib files.

use crate::lexer::{Lexer, LoadedSource, Peekable, TokenType};

/// A node in an Abstract Syntax Tree.
#[derive(Debug, PartialEq, Eq)]
pub enum ASTNode {
    /// The root of the AST.
    Root(Vec<ASTNode>),
    /// An HTML template.
    Template(String),
    /// A style block.
    Style(String),
    /// A script block.
    Script(Vec<ASTNode>),
    /// A comment.
    Comment(String),
    /// An enum.
    Enum(String, Vec<ASTNode>),
    /// An enum value.
    EnumValue(String),
}

/// Parses a template block.
///
/// # Examples
///
/// ```
/// use jib::parser::{parse, ASTNode};
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<template><p>Hi!</p></template>".to_string());
/// let ast_root = parse(&mut lexer).unwrap();
/// assert_eq!(
///     ast_root,
///     ASTNode::Root(vec![
///         ASTNode::Template("<p>Hi!</p>".to_string())
///     ])
/// );
/// ```
fn parse_template_block(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let mut open_blocks = 1;
    let mut value = String::new();

    while open_blocks > 0 {
        let token = tokens
            .next()
            .ok_or_else(|| (None, "Missing closing </template> tag.".to_string()))?;

        match token.token_type {
            TokenType::TagTemplateStart => open_blocks += 1,
            TokenType::TagTemplateEnd => open_blocks -= 1,
            _ => {}
        }

        if open_blocks > 0 {
            value.push_str(&token.value);
        }
    }

    Ok(Some(ASTNode::Template(value)))
}

/// Parses a style block.
///
/// # Examples
///
/// ```
/// use jib::parser::{parse, ASTNode};
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<style>p { left: 0 }</style>".to_string());
/// let ast_root = parse(&mut lexer).unwrap();
/// assert_eq!(
///     ast_root,
///     ASTNode::Root(vec![
///         ASTNode::Style("p { left: 0 }".to_string())
///     ])
/// );
/// ```
fn parse_style_block(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let mut open_blocks = 1;
    let mut value = String::new();

    while open_blocks > 0 {
        let token = tokens
            .next()
            .ok_or_else(|| (None, "Missing closing </style> tag.".to_string()))?;

        match token.token_type {
            TokenType::TagStyleStart => open_blocks += 1,
            TokenType::TagStyleEnd => open_blocks -= 1,
            _ => {}
        }

        if open_blocks > 0 {
            value.push_str(&token.value);
        }
    }

    Ok(Some(ASTNode::Style(value)))
}

/// Parses an enum declaration.
///
/// # Examples
///
/// ```
/// use jib::parser::{parse, ASTNode};
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<script>enum Foo = { Bar|Baz }</script>".to_string());
/// let ast_root = parse(&mut lexer).unwrap();
/// assert_eq!(
///     ast_root,
///     ASTNode::Root(vec![
///         ASTNode::Script(vec![
///             ASTNode::Enum(
///                 "Foo".to_string(),
///                 vec![
///                     ASTNode::EnumValue("Bar".to_string()),
///                     ASTNode::EnumValue("Baz".to_string())
///                 ]
///             )
///         ])
///     ])
/// );
/// ```
fn parse_enum(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    tokens.expect_token(TokenType::Keyword)?;
    tokens.flush_whitespace();
    let name_token = tokens.expect_token(TokenType::Text)?;
    tokens.flush_whitespace();
    tokens.expect_token(TokenType::Equal)?;
    tokens.flush_whitespace();
    tokens.expect_token(TokenType::CurlyBracketOpen)?;

    let mut enum_values = Vec::new();
    loop {
        tokens.flush_whitespace();
        let enum_value_token = tokens.expect_token(TokenType::Text)?;
        enum_values.push(ASTNode::EnumValue(enum_value_token.value));
        tokens.flush_whitespace();
        let delimiter_token = tokens.next().ok_or_else(|| {
            (
                Some(enum_value_token.line_number),
                "Expected `}` or `|`".to_string(),
            )
        })?;
        match delimiter_token.token_type {
            TokenType::CurlyBracketClose => {
                break;
            }
            TokenType::Pipe => {
                continue;
            }
            _ => {
                return Err((Some(name_token.line_number), "Syntax error".to_string()));
            }
        }
    }
    Ok(Some(ASTNode::Enum(name_token.value, enum_values)))
}

/// Parses a comment.
///
/// # Examples
///
/// ```
/// use jib::parser::{parse, ASTNode};
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<script># Foo bar baz.\n</script>".to_string());
/// let ast_root = parse(&mut lexer).unwrap();
/// assert_eq!(
///     ast_root,
///     ASTNode::Root(vec![
///         ASTNode::Script(vec![
///             ASTNode::Comment("Foo bar baz.".to_string())
///         ])
///     ])
/// );
/// ```
fn parse_comment(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let token = tokens.expect_token(TokenType::Comment)?;

    Ok(Some(ASTNode::Comment(token.value)))
}

fn parse_statement(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let token = tokens.peek().expect("should have token");

    match token.token_type {
        TokenType::Comment => parse_comment(tokens),
        TokenType::Keyword => match token.value.as_str() {
            "enum" => parse_enum(tokens),
            &_ => todo!(),
        },
        _ => {
            tokens.next();
            Ok(None)
            // TODO: replace with following Err
            //Err((Some(token.line_number), "Syntax error".to_string()))
        }
    }
}

/// Parses a script block.
///
/// # Examples
///
/// ```
/// use jib::parser::{parse, ASTNode};
/// use jib::lexer::Lexer;
///
/// let mut lexer = Lexer::new().load_source("<script></script>".to_string());
/// let ast_root = parse(&mut lexer).unwrap();
/// assert_eq!(
///     ast_root,
///     ASTNode::Root(vec![
///         ASTNode::Script(vec![])
///     ])
/// );
/// ```
fn parse_script_block(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let mut statements = Vec::new();
    loop {
        let token = tokens
            .peek()
            .ok_or_else(|| (None, "Missing closing </script> tag.".to_string()))?;

        match token.token_type {
            TokenType::TagScriptEnd => {
                tokens.next();
                break;
            }
            TokenType::Whitespace | TokenType::Newline => {
                tokens.next();
            }
            _ => {
                if let Some(statement) = parse_statement(tokens)? {
                    statements.push(statement);
                }
            }
        }
    }
    Ok(Some(ASTNode::Script(statements)))
}

fn parse_html_block(
    tokens: &mut Lexer<LoadedSource>,
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    tokens.flush_whitespace();

    let next_token = tokens
        .next()
        .ok_or_else(|| (None, "Unexpected end of file".to_string()))?;

    match next_token.token_type {
        TokenType::TagTemplateStart => parse_template_block(tokens),
        TokenType::TagStyleStart => parse_style_block(tokens),
        TokenType::TagScriptStart => parse_script_block(tokens),
        _ => Err((
            Some(next_token.line_number),
            "Expected a <template>, <style> or <script> block".to_string(),
        )),
    }
}

/// Takes tokens and restructures them into an Abstract Syntax Tree.
pub fn parse(tokens: &mut Lexer<LoadedSource>) -> Result<ASTNode, (Option<usize>, String)> {
    let mut html_blocks = Vec::new();
    while tokens.peek().is_some() {
        if let Some(html_block) = parse_html_block(tokens)? {
            html_blocks.push(html_block);
        }
    }
    Ok(ASTNode::Root(html_blocks))
}
