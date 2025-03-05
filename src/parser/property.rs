use crate::{
    ast::{
        ArrayCell, ArrayValue, BoolPropertyDefinition, IntValue, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyName, PropertyValue, PropertyValues, StringValue,
    },
    lexer::TokenKind,
};

use super::{utils::parse_list, ParseError, Parser};

// TODO: parse /delete-property/ for false case
pub(crate) fn parse_property(p: &mut Parser) -> Result<PropertyDefinition, ParseError> {
    let prop = if p.nth_at(1, TokenKind::SEMICOLON) {
        PropertyDefinition::Bool(parse_boolean_property(p)?)
    } else {
        PropertyDefinition::NonBool(parse_non_bool_property(p)?)
    };
    Ok(prop)
}

fn parse_boolean_property(p: &mut Parser) -> Result<BoolPropertyDefinition, ParseError> {
    let start = p.start();
    let name = parse_property_name(p)?;
    p.expect(TokenKind::SEMICOLON)?;
    Ok(BoolPropertyDefinition {
        name,
        range: p.end(start),
    })
}

fn parse_non_bool_property(p: &mut Parser) -> Result<NonBoolPropertyDefinition, ParseError> {
    let start = p.start();
    let name = parse_property_name(p)?;
    p.expect(TokenKind::EQUAL)?;
    let values = parse_non_bool_property_values(p)?;
    Ok(NonBoolPropertyDefinition {
        name,
        values,
        range: p.end(start),
    })
}

fn parse_property_name(p: &mut Parser) -> Result<PropertyName, ParseError> {
    let start = p.start();

    p.expect(TokenKind::NAME)?;
    Ok(PropertyName {
        range: p.end(start),
    })
}

fn parse_non_bool_property_values(p: &mut Parser) -> Result<PropertyValues, ParseError> {
    let start = p.start();
    let values = parse_list(
        p,
        parse_property_value,
        TokenKind::SEMICOLON,
        Some(TokenKind::COMMA),
    )?;
    p.expect(TokenKind::SEMICOLON)?;
    Ok(PropertyValues {
        values,
        range: p.end(start),
    })
}

fn parse_property_value(p: &mut Parser) -> Result<PropertyValue, ParseError> {
    match p.current_token_kind() {
        TokenKind::L_ANGLE => Ok(PropertyValue::Array(parse_array_value(p)?)),
        TokenKind::STRING => Ok(PropertyValue::String(parse_string_value(p)?)),
        _ => Err(ParseError::new(format!(
            "Expected {} or {}, but found {}",
            TokenKind::L_ANGLE,
            TokenKind::STRING,
            p.current_token_kind()
        ))),
    }
}

fn parse_array_value(p: &mut Parser) -> Result<ArrayValue, ParseError> {
    let start = p.start();
    p.expect(TokenKind::L_ANGLE)?;
    let cells = parse_list(p, parse_array_cell, TokenKind::R_ANGLE, None)?;
    p.expect(TokenKind::R_ANGLE)?;
    Ok(ArrayValue {
        cells,
        range: p.end(start),
    })
}

fn parse_array_cell(p: &mut Parser) -> Result<ArrayCell, ParseError> {
    Ok(ArrayCell::Int(parse_int_array_cell(p)?))
}

fn parse_int_array_cell(p: &mut Parser) -> Result<IntValue, ParseError> {
    assert!(p.at(TokenKind::INT));
    let start = p.start();
    p.bump(TokenKind::INT);
    Ok(IntValue {
        range: p.end(start),
    })
}

fn parse_string_value(p: &mut Parser) -> Result<StringValue, ParseError> {
    let start = p.start();
    p.expect(TokenKind::STRING)?;
    Ok(StringValue {
        range: p.end(start),
    })
}
