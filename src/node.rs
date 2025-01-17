use nom::{
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::{anychar, multispace0},
    combinator::{cut, opt, verify},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    label: Option<String>,
    name: String,
    address: Option<String>,
    children: Vec<Node>,
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let inner_body_parser = many0(delimited(multispace0, parse_node, multispace0));
    let body_parser = terminated(
        delimited(
            preceded(multispace0, tag("{")),
            inner_body_parser,
            terminated(tag("}"), multispace0),
        ),
        tag(";"),
    );
    let mut parser = tuple((
        opt(terminated(parse_label, multispace0)),
        parse_node_name,
        opt(parse_address),
        body_parser,
    ));
    let (rest, (label, name, address, children)) = parser(input)?;
    Ok((
        rest,
        Node {
            label: label.map(|label| label.to_string()),
            name,
            address: address.map(|address| address.to_string()),
            children,
        },
    ))
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    let mut parser = terminated(take_while_m_n(1, 31, is_valid_label_character), tag(":"));
    parser(input)
}

fn parse_property_name(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 31, is_valid_property_name_character)(input)
}

fn parse_node_name(input: &str) -> IResult<&str, String> {
    let first_character_parser = verify(anychar, |c| c.is_alphabetic());
    let rest_node_name = take_while_m_n(0, 30, is_valid_node_name_character);
    let mut parser = pair(first_character_parser, rest_node_name);
    let (rest, (first_character, rest_node_name)) = parser(input)?;

    Ok((rest, String::from(first_character) + rest_node_name))
}

fn parse_address(input: &str) -> IResult<&str, &str> {
    let mut parser = preceded(tag("@"), cut(take_while1(is_valid_node_name_character)));
    parser(input)
}

const VALID_NODE_NAME_CHAR: &str = ",._+-";

fn is_valid_node_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_NODE_NAME_CHAR.contains(c)
}

fn is_valid_label_character(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

const VALID_PROPERTY_NAME_CHAR: &str = ",._+?#-";

fn is_valid_property_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_PROPERTY_NAME_CHAR.contains(c)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_node_correctly() {
        assert_eq!(
            parse_node("node {};"),
            Ok((
                "",
                Node {
                    label: None,
                    name: "node".to_string(),
                    address: None,
                    children: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_node_with_label_correctly() {
        assert_eq!(
            parse_node("label: node {};"),
            Ok((
                "",
                Node {
                    label: Some("label".to_string()),
                    name: "node".to_string(),
                    address: None,
                    children: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_node_with_address_correctly() {
        assert_eq!(
            parse_node("label: node@12 {};"),
            Ok((
                "",
                Node {
                    label: Some("label".to_string()),
                    name: "node".to_string(),
                    address: Some("12".to_string()),
                    children: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_node_with_empty_address_fail() {
        assert!(parse_node("node@ {};").is_err());
    }

    #[test]
    fn parse_node_with_children_correctly() {
        assert_eq!(
            parse_node(
                r#"node {
    child1 {};
    child2 {};
};"#
            ),
            Ok((
                "",
                Node {
                    label: None,
                    name: "node".to_string(),
                    address: None,
                    children: vec![
                        Node {
                            label: None,
                            name: "child1".to_string(),
                            address: None,
                            children: Vec::new()
                        },
                        Node {
                            label: None,
                            name: "child2".to_string(),
                            address: None,
                            children: Vec::new()
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn parse_node_with_nested_children_correctly() {
        assert_eq!(
            parse_node(
                r#"node {
    label1: child1 {};
    label2: child2@address2 {
        label21: child21 {};
    };
};"#
            ),
            Ok((
                "",
                Node {
                    label: None,
                    name: "node".to_string(),
                    address: None,
                    children: vec![
                        Node {
                            label: Some("label1".to_string()),
                            name: "child1".to_string(),
                            address: None,
                            children: Vec::new()
                        },
                        Node {
                            label: Some("label2".to_string()),
                            name: "child2".to_string(),
                            address: Some("address2".to_string()),
                            children: vec![Node {
                                label: Some("label21".to_string()),
                                name: "child21".to_string(),
                                address: None,
                                children: Vec::new()
                            }]
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn parse_property_name_correctly() {
        assert_eq!(
            parse_property_name("ibm,ppc-interrupt-server#s"),
            Ok(("", "ibm,ppc-interrupt-server#s"))
        );
    }
}
