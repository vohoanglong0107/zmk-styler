use crate::lexer::TokenKind;

use super::{property::parse_property, utils::parse_list, Parser, SyntaxKind};

pub(crate) fn parse_node(p: &mut Parser) {
    let start = p.start();
    parse_label(p);
    parse_node_identifier(p);
    parse_node_body(p);
    p.end(start, SyntaxKind::NodeDefinition)
}

fn parse_label(p: &mut Parser) {
    if !p.nth_at(1, TokenKind::COLON) {
        return;
    }
    let start = p.start();
    p.expect(TokenKind::NAME);
    p.expect(TokenKind::COLON);
    p.end(start, SyntaxKind::Label)
}

fn parse_node_identifier(p: &mut Parser) {
    if p.at(TokenKind::ROOT) {
        parse_root_node_identifier(p)
    } else {
        parse_non_root_node_identifier(p)
    };
}

fn parse_root_node_identifier(p: &mut Parser) {
    let start = p.start();
    p.expect(TokenKind::ROOT);
    p.end(start, SyntaxKind::RootNodeIdentifier)
}

fn parse_non_root_node_identifier(p: &mut Parser) {
    let start = p.start();
    parse_node_name(p);
    parse_node_address(p);
    p.end(start, SyntaxKind::NonRootNodeIdentifier)
}

fn parse_node_name(p: &mut Parser) {
    let start = p.start();
    p.expect(TokenKind::NAME);
    p.end(start, SyntaxKind::NodeName)
}

fn parse_node_address(p: &mut Parser) {
    if !p.at(TokenKind::AT) {
        return;
    }
    let start = p.start();
    p.bump(TokenKind::AT);
    if p.at(TokenKind::INT) {
        p.bump(TokenKind::INT)
    } else {
        p.expect(TokenKind::NAME)
    };
    p.end(start, SyntaxKind::NodeAddress)
}

fn parse_node_body(p: &mut Parser) {
    let start = p.start();
    p.expect(TokenKind::L_CURLY);
    parse_node_body_entries(p);
    p.expect(TokenKind::R_CURLY);
    p.expect(TokenKind::SEMICOLON);
    p.end(start, SyntaxKind::NodeBody)
}

fn parse_node_body_entries(p: &mut Parser) {
    let start = p.start();
    parse_list(p, parse_node_body_entry, TokenKind::R_CURLY, None);
    p.end(start, SyntaxKind::NodeBodyEntries)
}

fn parse_node_body_entry(p: &mut Parser) {
    if is_at_node_property(p) {
        parse_property(p)
    } else if is_at_child_node(p) {
        parse_node(p)
    }
}

fn is_at_node_property(p: &Parser) -> bool {
    p.at(TokenKind::NAME) && (p.nth_at(1, TokenKind::SEMICOLON) || p.nth_at(1, TokenKind::EQUAL))
}

fn is_at_child_node(p: &Parser) -> bool {
    p.at(TokenKind::NAME) || p.at(TokenKind::ROOT)
}
