//! Parser module for Jib files.

use crate::lexer::{Peekable, Token, TokenType};

/// A node in an Abstract Syntax Tree.
#[derive(Debug)]
pub enum ASTNode {
    /// The root of the AST.
    Root(Vec<ASTNode>),
    /// An HTML template.
    Template(String),
    /// A style block.
    Style(String),
    /// A script block.
    Script(Vec<ASTNode>),
}

fn parse_template_block(
    tokens: &mut (impl Iterator<Item = Token> + Peekable<Token>),
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let mut open_blocks = 1;
    let mut value = String::new();

    while open_blocks > 0 {
        let token = tokens
            .next()
            .ok_or_else(|| (None, "Unexpected end of file".to_string()))?;

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

fn parse_style_block(
    tokens: &mut (impl Iterator<Item = Token> + Peekable<Token>),
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let mut open_blocks = 1;
    let mut value = String::new();

    while open_blocks > 0 {
        let token = tokens
            .next()
            .ok_or_else(|| (None, "Unexpected end of file".to_string()))?;

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

fn parse_script_block(
    tokens: &mut (impl Iterator<Item = Token> + Peekable<Token>),
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    while tokens.next().unwrap().token_type != TokenType::TagScriptEnd {
        // TODO: parse until end is reached
    }
    Ok(Some(ASTNode::Script(vec![])))
}

fn parse_html_block(
    tokens: &mut (impl Iterator<Item = Token> + Peekable<Token>),
) -> Result<Option<ASTNode>, (Option<usize>, String)> {
    let next_token = tokens
        .next()
        .ok_or_else(|| (None, "Unexpected end of file".to_string()))?;

    match next_token.token_type {
        TokenType::TagTemplateStart => parse_template_block(tokens),
        TokenType::TagStyleStart => parse_style_block(tokens),
        TokenType::TagScriptStart => parse_script_block(tokens),
        TokenType::Whitespace | TokenType::Newline => Ok(None),
        _ => Err((
            Some(next_token.line_number),
            "Expected a <template>, <style> or <script> block".to_string(),
        )),
    }
}

/// Takes tokens and restructures them into an Abstract Syntax Tree.
pub fn parse(
    tokens: &mut (impl Iterator<Item = Token> + Peekable<Token>),
) -> Result<ASTNode, (Option<usize>, String)> {
    let mut html_blocks = vec![];
    loop {
        if tokens.peek().is_none() {
            return Ok(ASTNode::Root(html_blocks));
        }

        if let Some(html_block) = parse_html_block(tokens)? {
            html_blocks.push(html_block);
        }
    }
}
