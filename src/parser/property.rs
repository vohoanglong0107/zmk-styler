use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{multispace0, multispace1, one_of},
    combinator::{map, map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use std::str::FromStr;

use crate::ast::{ArrayCell, NonBoolPropertyValue, Property, PropertyValue, PropertyValues};

// TODO: parse /delete-property/ for false case
pub(crate) fn parse_property(input: &str) -> IResult<&str, Property> {
    let mut parser = alt((parse_non_bool_property, parse_boolean_property));
    parser(input)
}

fn parse_boolean_property(input: &str) -> IResult<&str, Property> {
    let mut parser = terminated(parse_property_name, tag(";"));
    let (rest, name) = parser(input)?;
    Ok((
        rest,
        Property {
            name: name.to_string(),
            value: PropertyValue::Bool,
        },
    ))
}

fn parse_non_bool_property(input: &str) -> IResult<&str, Property> {
    let values_parser = alt((parse_non_bool_property_values,));
    let mut parser = terminated(
        separated_pair(
            parse_property_name,
            delimited(multispace0, tag("="), multispace0),
            values_parser,
        ),
        tag(";"),
    );
    let (rest, (name, values)) = parser(input)?;
    Ok((
        rest,
        Property {
            name: name.to_string(),
            value: PropertyValue::Values(values),
        },
    ))
}

fn parse_property_name(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 31, is_valid_property_name_character)(input)
}

fn parse_non_bool_property_values(input: &str) -> IResult<&str, PropertyValues> {
    let value_parser = alt((parse_array_value,));
    let mut parser = map(
        separated_list1(delimited(multispace0, tag(","), multispace0), value_parser),
        Into::into,
    );
    parser(input)
}

fn parse_array_value(input: &str) -> IResult<&str, NonBoolPropertyValue> {
    let array_cell_parser = alt((parse_int_array_cell,));
    let mut parser = map(
        delimited(
            tag("<"),
            separated_list1(multispace1, array_cell_parser),
            tag(">"),
        ),
        |array| NonBoolPropertyValue::Array(array.into()),
    );
    parser(input)
}

// https://learn.microsoft.com/en-us/cpp/c-language/c-integer-constants?view=msvc-170
fn parse_int_array_cell(input: &str) -> IResult<&str, ArrayCell> {
    let hex_parser = recognize(pair(
        alt((tag("0x"), tag("0X"))),
        many1(one_of("0123456789abcdefABCDEF")),
    ));
    let oct_parser = recognize(pair(tag("0"), many1(one_of("01234567"))));
    let dec_parser = recognize(pair(
        one_of("1234567"),
        many0(one_of("0123456789abcdefABCDEF")),
    ));
    let mut parser = map(alt((hex_parser, oct_parser, dec_parser)), |val: &str| {
        ArrayCell::Int(val.to_string())
    });
    parser(input)
}

const VALID_PROPERTY_NAME_CHAR: &str = ",._+?#-";

fn is_valid_property_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_PROPERTY_NAME_CHAR.contains(c)
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn parse_boolean_property_correctly() {
        assert_debug_snapshot!(
            parse_property("hold-trigger-on-release;"),
            @r#"
        Ok(
            (
                "",
                Property {
                    name: "hold-trigger-on-release",
                    value: Bool,
                },
            ),
        )
        "#
        )
    }

    #[test]
    fn parse_property_name_correctly() {
        assert_debug_snapshot!(
            parse_property_name("ibm,ppc-interrupt-server#s"),
            @r#"
        Ok(
            (
                "",
                "ibm,ppc-interrupt-server#s",
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_i32_array_property_correctly() {
        assert_debug_snapshot!(
            parse_property("an-array = <1 2 3>;"),
            @r#"
        Ok(
            (
                "",
                Property {
                    name: "an-array",
                    value: Values(
                        PropertyValues(
                            [
                                Array(
                                    ArrayValue(
                                        [
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
            ),
        )
        "#
        )
    }

    #[test]
    fn parse_i32_array_value_correctly() {
        assert_debug_snapshot!(
            parse_array_value("<1 20 330>"),
            @r#"
        Ok(
            (
                "",
                Array(
                    ArrayValue(
                        [
                            Int(
                                "1",
                            ),
                            Int(
                                "20",
                            ),
                            Int(
                                "330",
                            ),
                        ],
                    ),
                ),
            ),
        )
        "#
        )
    }

    #[test]
    fn parse_i32_array_cell_correctly() {
        assert_debug_snapshot!(
            parse_int_array_cell("123"),
            @r#"
        Ok(
            (
                "",
                Int(
                    "123",
                ),
            ),
        )
        "#
        );

        assert_debug_snapshot!(
            parse_int_array_cell("0x123cf"),
            @r#"
        Ok(
            (
                "",
                Int(
                    "0x123cf",
                ),
            ),
        )
        "#
        );

        assert_debug_snapshot!(
            parse_int_array_cell("0123"),
            @r#"
        Ok(
            (
                "",
                Int(
                    "0123",
                ),
            ),
        )
        "#
        )
    }
}
