use core::panic;
use std::cell::{Cell, RefCell};

use document::parse_document;

use crate::{
    ast::{AstNode, Document, SyntaxKind, SyntaxNodeBuilder},
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
    nodes: Vec<SyntaxNodeBuilder>,
    diasnostics: Vec<ParseError>,
}

impl<'src> Parser<'src> {
    pub(crate) fn new(lexer: BufferedLexer<'src>) -> Self {
        Self {
            lexer: RefCell::new(lexer),
            stuck_threshold: Cell::new(200),
            nodes: Vec::new(),
            diasnostics: Vec::new(),
        }
    }

    pub(super) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump(kind)
        } else {
            self.diasnostics.push(ParseError::new(
                format!("Expected {kind}, but found {}", self.current_token_kind()),
                self.lexer.get_mut().current_token_range(),
            ))
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

    pub(super) fn at_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|kind| self.at(*kind))
    }

    pub(super) fn nth_at(&self, pos: usize, kind: TokenKind) -> bool {
        let token = self.nth(pos);
        token.kind == kind
    }

    pub(super) fn bump(&mut self, kind: TokenKind) {
        let lexer = self.lexer.get_mut();
        let token = lexer.advance();

        assert_eq!(token.kind, kind);
        let current_node = self.nodes.last_mut().unwrap();
        current_node.push_token(token);
    }

    pub(super) fn bump_any(&mut self) {
        let lexer = self.lexer.get_mut();
        let token = lexer.advance();

        let current_node = self.nodes.last_mut().unwrap();
        current_node.push_token(token);
    }

    pub(super) fn start(&mut self) -> SourceIndex {
        self.nodes.push(SyntaxNodeBuilder::new());
        self.lexer.borrow_mut().current_token_start()
    }

    pub(super) fn end(&mut self, start: SourceIndex, kind: SyntaxKind) {
        let end = self.lexer.borrow_mut().last_token_end();
        let node_range = SourceRange::new(start, end);
        let mut current_node = self.nodes.pop().unwrap();
        current_node.kind(kind);
        current_node.range(node_range);
        if let Some(parent) = self.nodes.last_mut() {
            parent.push_node(current_node.build());
        } else {
            self.nodes.push(current_node);
        }
    }

    pub(super) fn finish(mut self) -> (Document, TokenSource, Vec<ParseError>) {
        assert_eq!(self.nodes.len(), 1);
        let token_source = TokenSource::new(self.lexer.into_inner().finish());
        let root = self.nodes.pop().unwrap();
        (
            Document::cast(&root.build()).unwrap(),
            token_source,
            self.diasnostics,
        )
    }
}

pub(crate) fn parse(source: &Source) -> (Document, TokenSource, Vec<ParseError>) {
    let lexer = Lexer::new(source);
    let lexer = BufferedLexer::new(lexer);
    let mut parser = Parser::new(lexer);
    parse_document(&mut parser);

    parser.finish()
}

#[derive(Debug)]
pub(crate) struct ParseError {
    pub(crate) msg: String,
    pub(crate) range: SourceRange,
}

impl ParseError {
    fn new(msg: impl ToString, range: SourceRange) -> Self {
        Self {
            msg: msg.to_string(),
            range,
        }
    }
}
