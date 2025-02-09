use crate::{
    ast::{ArrayCell, NonBoolPropertyValue, Property, PropertyValue, PropertyValues},
    lexer::TokenKind,
};

use super::{ParseError, Parser};

// TODO: parse /delete-property/ for false case
pub(crate) fn parse_property(p: &mut Parser) -> Result<Property, ParseError> {
    if p.nth_at(1, TokenKind::SEMICOLON) {
        parse_boolean_property(p)
    } else {
        parse_non_bool_property(p)
    }
}

fn parse_boolean_property(p: &mut Parser) -> Result<Property, ParseError> {
    let token = p.expect(TokenKind::NAME)?;
    p.expect(TokenKind::SEMICOLON)?;
    Ok(Property {
        name: token.text.to_string(),
        value: PropertyValue::Bool,
    })
}

fn parse_non_bool_property(p: &mut Parser) -> Result<Property, ParseError> {
    let name_token = p.expect(TokenKind::NAME)?;
    p.expect(TokenKind::EQUAL)?;

    let values = parse_non_bool_property_values(p)?;
    Ok(Property {
        name: name_token.text,
        value: PropertyValue::Values(values),
    })
}

fn parse_non_bool_property_values(p: &mut Parser) -> Result<PropertyValues, ParseError> {
    let mut values = Vec::new();

    let first_value = parse_non_bool_property_value(p)?;
    values.push(first_value);

    loop {
        if p.nth_at(0, TokenKind::SEMICOLON) {
            break;
        }
        p.expect(TokenKind::COMMA)?;
        if is_at_non_bool_property_value(p) {
            let value = parse_non_bool_property_value(p)?;
            values.push(value)
        } else {
            break;
        }
    }
    p.expect(TokenKind::SEMICOLON)?;
    Ok(values.into())
}

fn is_at_non_bool_property_value(p: &Parser) -> bool {
    p.nth_at(0, TokenKind::L_ANGLE) || p.nth_at(0, TokenKind::STRING)
}

fn parse_non_bool_property_value(p: &mut Parser) -> Result<NonBoolPropertyValue, ParseError> {
    let current_token = p.nth(0).ok_or(ParseError::new(format!(
        "Expected {} or {}, but found EOF",
        TokenKind::L_ANGLE,
        TokenKind::STRING
    )))?;
    match current_token.kind {
        TokenKind::L_ANGLE => parse_array_value(p),
        TokenKind::STRING => parse_string_value(p),
        _ => Err(ParseError::new(format!(
            "Expected {} or {}, but found {}",
            TokenKind::L_ANGLE,
            TokenKind::STRING,
            current_token.kind
        ))),
    }
}

fn parse_array_value(p: &mut Parser) -> Result<NonBoolPropertyValue, ParseError> {
    let mut array_value = Vec::new();
    p.expect(TokenKind::L_ANGLE)?;
    while let Ok(token) = p.expect(TokenKind::INT) {
        array_value.push(ArrayCell::Int(token.text));
    }
    p.expect(TokenKind::R_ANGLE)?;
    Ok(NonBoolPropertyValue::Array(array_value.into()))
}

fn parse_string_value(p: &mut Parser) -> Result<NonBoolPropertyValue, ParseError> {
    let token = p.expect(TokenKind::STRING)?;
    Ok(NonBoolPropertyValue::String(token.text.into()))
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn parse_boolean_property_correctly() {
        assert_debug_snapshot!(
            parse("hold-trigger-on-release;"),
            @r#"
        Ok(
            Property {
                name: "hold-trigger-on-release",
                value: Bool,
            },
        )
        "#
        )
    }

    #[test]
    fn parse_i32_array_property_correctly() {
        assert_debug_snapshot!(
            parse("an-array = <0 1 2 3>;"),
            @r#"
        Ok(
            Property {
                name: "an-array",
                value: Values(
                    PropertyValues(
                        [
                            Array(
                                ArrayValue(
                                    [
                                        Int(
                                            "0",
                                        ),
                                        Int(
                                            "1",
                                        ),
                                        Int(
                                            "2",
                                        ),
                                        Int(
                                            "3",
                                        ),
                                    ],
                                ),
                            ),
                        ],
                    ),
                ),
            },
        )
        "#
        )
    }

    #[test]
    fn parse_string_property_correctly() {
        assert_debug_snapshot!(
            parse(r#"compatible = "zmk,behavior-tap-dance";"#),
            @r#"
        Ok(
            Property {
                name: "compatible",
                value: Values(
                    PropertyValues(
                        [
                            String(
                                StringValue(
                                    "zmk,behavior-tap-dance",
                                ),
                            ),
                        ],
                    ),
                ),
            },
        )
        "#
        )
    }

    fn parse(s: &str) -> Result<Property, ParseError> {
        let mut parser = Parser::new(s);
        parse_property(&mut parser)
    }
}
