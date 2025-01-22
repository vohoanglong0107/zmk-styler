use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    sequence::terminated,
    IResult,
};

use crate::ast::{Property, PropertyValue};

pub(super) fn parse_property(input: &str) -> IResult<&str, Property> {
    let mut parser = alt((parse_boolean_property,));
    parser(input)
}

fn parse_boolean_property(input: &str) -> IResult<&str, Property> {
    // TODO: parse /delete-property/ for false case
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

fn parse_property_name(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 31, is_valid_property_name_character)(input)
}

const VALID_PROPERTY_NAME_CHAR: &str = ",._+?#-";

fn is_valid_property_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_PROPERTY_NAME_CHAR.contains(c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_boolean_property_correctly() {
        assert_eq!(
            parse_property("hold-trigger-on-release;"),
            Ok((
                "",
                Property {
                    name: "hold-trigger-on-release".to_string(),
                    value: PropertyValue::Bool
                }
            ))
        )
    }

    #[test]
    fn parse_property_name_correctly() {
        assert_eq!(
            parse_property_name("ibm,ppc-interrupt-server#s"),
            Ok(("", "ibm,ppc-interrupt-server#s"))
        );
    }
}
