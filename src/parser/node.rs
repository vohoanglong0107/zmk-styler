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

pub(crate) fn parse_node(input: &str) -> IResult<&str, Node> {
    let mut parser = tuple((
        opt(terminated(parse_label, multispace0)),
        alt((parse_root_node, parse_node_name)),
        opt(parse_address),
        // Todo: remove terminated after handling parsing multiple nodes in a file
        terminated(parse_node_body, multispace0),
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

fn parse_root_node(input: &str) -> IResult<&str, String> {
    let parser = tag("/");
    let (rest, root) = parser(input)?;
    Ok((rest, root.to_string()))
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
            delimited(multispace0, tag("{"), multispace0),
            inner_body_parser,
            delimited(multispace0, tag("}"), multispace0),
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
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn parse_node_correctly() {
        assert_debug_snapshot!(parse_node("node {};"), @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "node",
                    address: None,
                    children: [],
                    properties: [],
                },
            ),
        )
        "#);
    }

    #[test]
    fn parse_root_node_correctly() {
        assert_debug_snapshot!(
            parse_node("/ {};"),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "/",
                    address: None,
                    children: [],
                    properties: [],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_label_correctly() {
        assert_debug_snapshot!(
            parse_node("label: node {};"),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: Some(
                        "label",
                    ),
                    name: "node",
                    address: None,
                    children: [],
                    properties: [],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_address_correctly() {
        assert_debug_snapshot!(
            parse_node("label: node@12 {};"),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: Some(
                        "label",
                    ),
                    name: "node",
                    address: Some(
                        "12",
                    ),
                    children: [],
                    properties: [],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_empty_address_fail() {
        assert!(parse_node("node@ {};").is_err());
    }

    #[test]
    fn parse_node_with_children_correctly() {
        assert_debug_snapshot!(
            parse_node(
                r#"node {
    child1 {};
    child2 {};
};"#
            ),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "node",
                    address: None,
                    children: [
                        Node {
                            label: None,
                            name: "child1",
                            address: None,
                            children: [],
                            properties: [],
                        },
                        Node {
                            label: None,
                            name: "child2",
                            address: None,
                            children: [],
                            properties: [],
                        },
                    ],
                    properties: [],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_nested_children_correctly() {
        assert_debug_snapshot!(
            parse_node(
                r#"node {
    label1: child1 {};
    label2: child2@address2 {
        label21: child21 {};
    };
};"#
            ),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "node",
                    address: None,
                    children: [
                        Node {
                            label: Some(
                                "label1",
                            ),
                            name: "child1",
                            address: None,
                            children: [],
                            properties: [],
                        },
                        Node {
                            label: Some(
                                "label2",
                            ),
                            name: "child2",
                            address: Some(
                                "address2",
                            ),
                            children: [
                                Node {
                                    label: Some(
                                        "label21",
                                    ),
                                    name: "child21",
                                    address: None,
                                    children: [],
                                    properties: [],
                                },
                            ],
                            properties: [],
                        },
                    ],
                    properties: [],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_properties_correctly() {
        assert_debug_snapshot!(
            parse_node(
                r#"node {
    property-one;
    label1: child1 {};
};"#
            ),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "node",
                    address: None,
                    children: [
                        Node {
                            label: Some(
                                "label1",
                            ),
                            name: "child1",
                            address: None,
                            children: [],
                            properties: [],
                        },
                    ],
                    properties: [
                        Property {
                            name: "property-one",
                            value: Bool,
                        },
                    ],
                },
            ),
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_empty_body_correctly() {
        assert_debug_snapshot!(
            parse_node(
                r#"/ {
    child1 {
    };
};"#
            ),
            @r#"
        Ok(
            (
                "",
                Node {
                    label: None,
                    name: "/",
                    address: None,
                    children: [
                        Node {
                            label: None,
                            name: "child1",
                            address: None,
                            children: [],
                            properties: [],
                        },
                    ],
                    properties: [],
                },
            ),
        )
        "#
        );
    }
}
