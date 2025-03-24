use crate::lexer::TokenKind;

use super::{node::parse_node, utils::parse_list, Parser, SyntaxKind};

pub(crate) fn parse_document(p: &mut Parser) {
    let start = p.start();
    parse_list(p, parse_statement, TokenKind::EOF, None);
    p.end(start, SyntaxKind::Document)
}

pub(crate) fn parse_statement(p: &mut Parser) {
    parse_node(p)
}
