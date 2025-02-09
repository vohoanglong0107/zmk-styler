use core::panic;
use std::cell::Cell;

use node::parse_node;
use thiserror::Error;

use crate::{
    ast::Node,
    lexer::{lex_tokens, Token, TokenKind},
};

pub(crate) mod node;
pub(crate) mod property;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    current_positions: usize,
    stuck_threshold: Cell<u32>,
}

impl Parser {
    pub(crate) fn new(input: &str) -> Self {
        let tokens = lex_tokens(input);

        // TODO: handle comments
        let tokens = tokens
            .into_iter()
            .filter(|token| {
                !matches!(
                    token.kind,
                    TokenKind::B_COMMENT
                        | TokenKind::S_COMMENT
                        | TokenKind::NEW_LINE
                        | TokenKind::SPACE
                )
            })
            .collect();
        Self {
            tokens,
            current_positions: 0,
            stuck_threshold: Cell::new(200),
        }
    }
    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let Some(token) = self.next_non_trivia() else {
            return Err(ParseError::new(format!("Expected {kind}, but found EOF")));
        };
        if token.kind == kind {
            self.advance();
            Ok(token.to_owned())
        } else {
            Err(ParseError::new(format!(
                "Expected {kind}, but found {}",
                token.kind
            )))
        }
    }

    pub(super) fn nth(&self, pos: usize) -> Option<Token> {
        let stuck_threshold = self.stuck_threshold.get();
        if stuck_threshold == 0 {
            panic!("The parser is likely stuck");
        }
        self.stuck_threshold.set(stuck_threshold - 1);
        self.tokens.get(self.current_positions + pos).cloned()
    }

    pub(super) fn nth_at(&self, pos: usize, kind: TokenKind) -> bool {
        let Some(token) = self.nth(pos) else {
            return false;
        };
        token.kind == kind
    }

    pub(super) fn next_non_trivia(&mut self) -> Option<Token> {
        while let Some(token) = self.nth(0) {
            if !token.is_trivia() {
                return Some(token);
            }
            self.advance();
        }
        None
    }

    pub(super) fn advance(&mut self) {
        self.stuck_threshold.set(200);
        self.current_positions += 1
    }
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub(crate) struct ParseError {
    msg: String,
}

impl ParseError {
    fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

pub(crate) fn parse(input: &str) -> Result<Node, ParseError> {
    let mut parser = Parser::new(input);
    parse_node(&mut parser)
}
