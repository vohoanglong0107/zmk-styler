use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::{anychar, multispace0},
    combinator::{cut, opt, verify},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::ast::{Node, Property};

use super::property::parse_property;

fn parse_node(input: &str) -> IResult<&str, Node> {
    let mut parser = tuple((
        opt(terminated(parse_label, multispace0)),
        parse_node_name,
        opt(parse_address),
        parse_node_body,
    ));
    let (rest, (label, name, address, (properties, children))) = parser(input)?;
    Ok((
        rest,
        Node {
            label: label.map(|label| label.to_string()),
            name,
            address: address.map(|address| address.to_string()),
            children,
            properties,
        },
    ))
}

fn parse_label(input: &str) -> IResult<&str, &str> {
    let mut parser = terminated(take_while_m_n(1, 31, is_valid_label_character), tag(":"));
    parser(input)
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

fn parse_node_body(input: &str) -> IResult<&str, (Vec<Property>, Vec<Node>)> {
    let inner_body_parser = many0(delimited(
        multispace0,
        alt((
            parse_node_body_node_definition,
            parse_node_body_property_definition,
        )),
        multispace0,
    ));
    let mut parser = terminated(
        delimited(
            preceded(multispace0, tag("{")),
            inner_body_parser,
            terminated(tag("}"), multispace0),
        ),
        tag(";"),
    );
    let (rest, body) = parser(input)?;
    let mut children = Vec::new();
    let mut properties = Vec::new();
    for definition in body {
        match definition {
            NodeBodyDefinition::Node(node) => children.push(node),
            NodeBodyDefinition::Property(property) => properties.push(property),
        }
    }
    Ok((rest, (properties, children)))
}

enum NodeBodyDefinition {
    Node(Node),
    Property(Property),
}

fn parse_node_body_node_definition(input: &str) -> IResult<&str, NodeBodyDefinition> {
    let (rest, node) = parse_node(input)?;
    Ok((rest, NodeBodyDefinition::Node(node)))
}

fn parse_node_body_property_definition(input: &str) -> IResult<&str, NodeBodyDefinition> {
    let (rest, property) = parse_property(input)?;
    Ok((rest, NodeBodyDefinition::Property(property)))
}

const VALID_NODE_NAME_CHAR: &str = ",._+-";

fn is_valid_node_name_character(c: char) -> bool {
    c.is_alphanumeric() || VALID_NODE_NAME_CHAR.contains(c)
}

fn is_valid_label_character(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod test {
    use crate::ast::PropertyValue;

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
                    children: Vec::new(),
                    properties: Vec::new()
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
                    children: Vec::new(),
                    properties: Vec::new()
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
                    children: Vec::new(),
                    properties: Vec::new()
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
                            children: Vec::new(),
                            properties: Vec::new()
                        },
                        Node {
                            label: None,
                            name: "child2".to_string(),
                            address: None,
                            children: Vec::new(),
                            properties: Vec::new()
                        }
                    ],
                    properties: Vec::new()
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
                            children: Vec::new(),
                            properties: Vec::new()
                        },
                        Node {
                            label: Some("label2".to_string()),
                            name: "child2".to_string(),
                            address: Some("address2".to_string()),
                            children: vec![Node {
                                label: Some("label21".to_string()),
                                name: "child21".to_string(),
                                address: None,
                                children: Vec::new(),
                                properties: Vec::new()
                            }],
                            properties: Vec::new()
                        }
                    ],
                    properties: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn parse_node_with_properties_correctly() {
        assert_eq!(
            parse_node(
                r#"node {
    property-one;
    label1: child1 {};
};"#
            ),
            Ok((
                "",
                Node {
                    label: None,
                    name: "node".to_string(),
                    address: None,
                    children: vec![Node {
                        label: Some("label1".to_string()),
                        name: "child1".to_string(),
                        address: None,
                        children: Vec::new(),
                        properties: Vec::new()
                    },],
                    properties: vec![Property {
                        name: "property-one".to_string(),
                        value: PropertyValue::Bool
                    }]
                }
            ))
        );
    }
}
