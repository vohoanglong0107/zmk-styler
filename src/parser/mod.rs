use core::panic;
use std::cell::{Cell, RefCell};

use document::parse_document;
use thiserror::Error;

use crate::{
    ast::Document,
    lexer::{BufferedLexer, Lexer, Token, TokenKind},
    source::{Source, SourceIndex, SourceRange},
    token_source::TokenSource,
};

mod document;
mod node;
mod property;
mod utils;

pub(crate) struct Parser<'src> {
    lexer: RefCell<BufferedLexer<'src>>,
    stuck_threshold: Cell<u32>,
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub(crate) struct ParseError {
    msg: String,
}

impl<'src> Parser<'src> {
    pub(crate) fn new(lexer: BufferedLexer<'src>) -> Self {
        Self {
            lexer: RefCell::new(lexer),
            stuck_threshold: Cell::new(200),
        }
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.at(kind) {
            self.bump(kind);
            Ok(())
        } else {
            Err(ParseError::new(format!(
                "Expected {kind}, but found {}",
                self.current_token_kind()
            )))
        }
    }

    pub(super) fn nth(&self, pos: usize) -> Token {
        let stuck_threshold = self.stuck_threshold.get();
        if stuck_threshold == 0 {
            panic!("The parser is likely stuck");
        }
        self.stuck_threshold.set(stuck_threshold - 1);
        let mut lexer = self.lexer.borrow_mut();
        lexer.nth(pos)
    }

    pub(super) fn current_token_kind(&self) -> TokenKind {
        let token = self.nth(0);
        token.kind
    }

    pub(super) fn at(&self, kind: TokenKind) -> bool {
        self.nth_at(0, kind)
    }

    pub(super) fn nth_at(&self, pos: usize, kind: TokenKind) -> bool {
        let token = self.nth(pos);
        token.kind == kind
    }

    pub(super) fn bump(&mut self, kind: TokenKind) {
        let mut lexer = self.lexer.borrow_mut();
        let token = lexer.advance();

        assert_eq!(token.kind, kind);
    }

    pub(super) fn start(&self) -> SourceIndex {
        self.lexer.borrow_mut().current_token_start()
    }

    pub(super) fn end(&self, start: SourceIndex) -> SourceRange {
        let end = self.lexer.borrow_mut().last_token_end();
        SourceRange::new(start, end)
    }

    pub(super) fn finish(self) -> TokenSource {
        TokenSource::new(self.lexer.into_inner().finish())
    }
}

pub(crate) fn parse(source: &Source) -> Result<(Document, TokenSource), ParseError> {
    let lexer = Lexer::new(source);
    let lexer = BufferedLexer::new(lexer);
    let mut parser = Parser::new(lexer);
    let doc = parse_document(&mut parser)?;
    let comments = parser.finish();
    Ok((doc, comments))
}

impl ParseError {
    fn new(msg: impl ToString) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}
