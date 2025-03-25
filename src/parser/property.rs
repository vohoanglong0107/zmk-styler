use crate::lexer::TokenKind;

use super::{utils::parse_list, Parser, SyntaxKind};

// TODO: parse /delete-property/ for false case
pub(crate) fn parse_property(p: &mut Parser) {
    if p.nth_at(1, TokenKind::SEMICOLON) {
        parse_boolean_property(p)
    } else {
        parse_non_bool_property(p)
    };
}

fn parse_boolean_property(p: &mut Parser) {
    let start = p.start();
    parse_property_name(p);
    p.expect(TokenKind::SEMICOLON);
    p.end(start, SyntaxKind::BoolPropertyDefinition)
}

fn parse_non_bool_property(p: &mut Parser) {
    let start = p.start();
    parse_property_name(p);
    p.expect(TokenKind::EQUAL);
    parse_property_values(p);
    p.end(start, SyntaxKind::NonBoolPropertyDefinition)
}

fn parse_property_name(p: &mut Parser) {
    let start = p.start();

    p.expect(TokenKind::NAME);
    p.end(start, SyntaxKind::PropertyName)
}

fn parse_property_values(p: &mut Parser) {
    let start = p.start();
    parse_list(
        p,
        parse_property_value,
        is_at_property_value,
        TokenKind::SEMICOLON,
        Some(TokenKind::COMMA),
        is_at_property_value_recovered,
    );
    p.expect(TokenKind::SEMICOLON);
    p.end(start, SyntaxKind::PropertyValues)
}

fn parse_property_value(p: &mut Parser) {
    match p.current_token_kind() {
        TokenKind::L_ANGLE => parse_array_value(p),
        TokenKind::STRING => parse_string_value(p),
        _ => {}
    }
}

fn parse_array_value(p: &mut Parser) {
    let start = p.start();
    p.expect(TokenKind::L_ANGLE);
    parse_list(
        p,
        parse_array_cell,
        is_at_array_cell,
        TokenKind::R_ANGLE,
        None,
        is_at_array_cell_recovered,
    );
    p.expect(TokenKind::R_ANGLE);
    p.end(start, SyntaxKind::ArrayValue)
}

fn parse_array_cell(p: &mut Parser) {
    parse_int_cell(p)
}

fn parse_int_cell(p: &mut Parser) {
    let start = p.start();
    p.bump(TokenKind::INT);
    p.end(start, SyntaxKind::IntCell)
}

fn parse_string_value(p: &mut Parser) {
    let start = p.start();
    p.expect(TokenKind::STRING);
    p.end(start, SyntaxKind::StringValue)
}

fn is_at_property_value(p: &Parser) -> bool {
    matches!(
        p.current_token_kind(),
        TokenKind::L_ANGLE | TokenKind::STRING
    )
}

fn is_at_array_cell(p: &Parser) -> bool {
    p.at(TokenKind::INT)
}

fn is_at_property_value_recovered(p: &Parser) -> bool {
    is_at_property_value(p) || p.at(TokenKind::SEMICOLON)
}

fn is_at_array_cell_recovered(p: &Parser) -> bool {
    is_at_array_cell(p) || p.at(TokenKind::R_ANGLE)
}
