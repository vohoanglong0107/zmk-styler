use nom::{
    bytes::complete::{take_while1, take_while_m_n},
    character::complete::{anychar, char},
    combinator::{cut, opt, verify},
    sequence::{pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    name: String,
    address: Option<Address>,
}

#[derive(Debug, PartialEq, Eq)]
struct Address {
    at_sign: char,
    address: String,
}

fn parse_node_name(input: &str) -> IResult<&str, Node> {
    let first_character_parser = verify(anychar, |c| c.is_alphabetic());
    let rest_node_name = take_while_m_n(0, 30, is_valid_node_name_character);
    let mut parser = tuple((first_character_parser, rest_node_name, opt(parse_address)));
    let (rest, (first_character, rest_node_name, address)) = parser(input)?;

    Ok((
        rest,
        Node {
            name: String::from(first_character) + rest_node_name,
            address,
        },
    ))
}

fn parse_property_name(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 31, is_valid_property_name_character)(input)
}

fn parse_address(input: &str) -> IResult<&str, Address> {
    let mut parser = pair(char('@'), cut(take_while1(is_valid_node_name_character)));
    let (rest, (at, address)) = parser(input)?;
    Ok((
        rest,
        Address {
            at_sign: at,
            address: address.to_string(),
        },
    ))
}

const VALID_NODE_NAME_CHAR: &str = ",._+-";

fn is_valid_node_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_NODE_NAME_CHAR.contains(c)
}

const VALID_PROPERTY_NAME_CHAR: &str = ",._+?#-";

fn is_valid_property_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_PROPERTY_NAME_CHAR.contains(c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_node_name_correctly() {
        assert_eq!(
            parse_node_name("node"),
            Ok((
                "",
                Node {
                    name: "node".to_string(),
                    address: None
                }
            ))
        );
    }

    #[test]
    fn parse_node_name_with_address_correctly() {
        assert_eq!(
            parse_node_name("node@12"),
            Ok((
                "",
                Node {
                    name: "node".to_string(),
                    address: Some(Address {
                        at_sign: '@',
                        address: "12".to_string()
                    })
                }
            ))
        );
    }

    #[test]
    fn parse_node_name_with_empty_address_fail() {
        assert!(parse_node_name("node@").is_err());
    }

    #[test]
    fn parse_property_name_correctly() {
        assert_eq!(
            parse_property_name("ibm,ppc-interrupt-server#s"),
            Ok(("", "ibm,ppc-interrupt-server#s"))
        );
    }
}
