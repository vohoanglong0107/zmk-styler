use crate::lexer::TokenKind;

use super::{
    node::{is_at_node, parse_node, END_OF_NODE_SET},
    utils::parse_list,
    Parser, SyntaxKind,
};

pub(crate) fn parse_document(p: &mut Parser) {
    let start = p.start();
    parse_list(
        p,
        parse_statement,
        is_at_statement,
        TokenKind::EOF,
        None,
        is_statement_recovered,
    );
    p.end(start, SyntaxKind::Document)
}

fn parse_statement(p: &mut Parser) {
    parse_node(p)
}

fn is_at_statement(p: &Parser) -> bool {
    is_at_node(p)
}

fn is_statement_recovered(p: &Parser) -> bool {
    p.at_any(&END_OF_NODE_SET) || p.at(TokenKind::EOF)
}
