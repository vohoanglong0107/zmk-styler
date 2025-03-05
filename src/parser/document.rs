use crate::{
    ast::{Document, Statement},
    lexer::TokenKind,
};

use super::{node::parse_node, utils::parse_list, ParseError, Parser};

pub(crate) fn parse_document(p: &mut Parser) -> Result<Document, ParseError> {
    let start = p.start();
    let statements = parse_list(p, parse_statement, TokenKind::EOF, None)?;
    Ok(Document {
        statements,

        range: p.end(start),
    })
}

pub(crate) fn parse_statement(p: &mut Parser) -> Result<Statement, ParseError> {
    Ok(Statement::Node(parse_node(p)?))
}
