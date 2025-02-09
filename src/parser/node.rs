use crate::{
    ast::{Node, Property},
    lexer::TokenKind,
};

use super::{property::parse_property, ParseError, Parser};

pub(crate) fn parse_node(p: &mut Parser) -> Result<Node, ParseError> {
    let label = parse_label(p)?;
    let identifier = parse_node_identifier(p)?;
    let (properties, children) = parse_node_body(p)?;
    Ok(Node {
        label,
        identifier,
        children,
        properties,
    })
}

fn parse_label(p: &mut Parser) -> Result<Option<String>, ParseError> {
    if p.nth_at(1, TokenKind::COLON) {
        let token = p.expect(TokenKind::NAME)?;
        p.expect(TokenKind::COLON)?;
        Ok(Some(token.text))
    } else {
        Ok(None)
    }
}

fn parse_node_identifier(p: &mut Parser) -> Result<String, ParseError> {
    let identifier = if p.nth_at(0, TokenKind::ROOT) {
        let root_token = p.expect(TokenKind::ROOT)?;
        root_token.text
    } else {
        let name = p.expect(TokenKind::NAME)?;
        if p.nth_at(0, TokenKind::AT) {
            p.expect(TokenKind::AT)?;
            let address = if p.nth_at(0, TokenKind::INT) {
                p.expect(TokenKind::INT)?
            } else {
                p.expect(TokenKind::NAME)?
            };
            format!("{}@{}", name.text, address.text)
        } else {
            name.text
        }
    };
    Ok(identifier)
}

fn parse_node_body(p: &mut Parser) -> Result<(Vec<Property>, Vec<Node>), ParseError> {
    p.expect(TokenKind::L_CURLY)?;
    let mut children = Vec::new();
    let mut properties = Vec::new();
    loop {
        if is_at_node_property(p) {
            let property = parse_property(p)?;
            properties.push(property)
        } else if is_at_child_node(p) {
            let node = parse_node(p)?;
            children.push(node)
        } else if p.nth_at(0, TokenKind::R_CURLY) {
            break;
        } else {
            return Err(ParseError::new(
                "Expected a property, a child node or a closing brace".to_string(),
            ));
        }
    }
    p.expect(TokenKind::R_CURLY)?;
    p.expect(TokenKind::SEMICOLON)?;
    Ok((properties, children))
}

fn is_at_node_property(p: &Parser) -> bool {
    p.nth_at(0, TokenKind::NAME)
        && (p.nth_at(1, TokenKind::SEMICOLON) || p.nth_at(1, TokenKind::EQUAL))
}

fn is_at_child_node(p: &Parser) -> bool {
    p.nth_at(0, TokenKind::NAME) || p.nth_at(0, TokenKind::ROOT)
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn parse_node_correctly() {
        assert_debug_snapshot!(parse("node {};"), @r#"
        Ok(
            Node {
                label: None,
                identifier: "node",
                children: [],
                properties: [],
            },
        )
        "#);
    }

    #[test]
    fn parse_root_node_correctly() {
        assert_debug_snapshot!(
            parse("/ {};"),
            @r#"
        Ok(
            Node {
                label: None,
                identifier: "/",
                children: [],
                properties: [],
            },
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_label_correctly() {
        assert_debug_snapshot!(
            parse("label: node {};"),
            @r#"
        Ok(
            Node {
                label: Some(
                    "label",
                ),
                identifier: "node",
                children: [],
                properties: [],
            },
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_address_correctly() {
        assert_debug_snapshot!(
            parse("label: node@12 {};"),
            @r#"
        Ok(
            Node {
                label: Some(
                    "label",
                ),
                identifier: "node@12",
                children: [],
                properties: [],
            },
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_empty_address_fail() {
        assert!(parse("node@ {};").is_err());
    }

    #[test]
    fn parse_node_with_children_correctly() {
        assert_debug_snapshot!(
            parse(
                r#"node {
    child1 {};
    child2 {};
};"#
            ),
            @r#"
        Ok(
            Node {
                label: None,
                identifier: "node",
                children: [
                    Node {
                        label: None,
                        identifier: "child1",
                        children: [],
                        properties: [],
                    },
                    Node {
                        label: None,
                        identifier: "child2",
                        children: [],
                        properties: [],
                    },
                ],
                properties: [],
            },
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_nested_children_correctly() {
        assert_debug_snapshot!(
            parse(
                r#"node {
    label1: child1 {};
    label2: child2@address2 {
        label21: child21 {};
    };
};"#
            ),
            @r#"
        Ok(
            Node {
                label: None,
                identifier: "node",
                children: [
                    Node {
                        label: Some(
                            "label1",
                        ),
                        identifier: "child1",
                        children: [],
                        properties: [],
                    },
                    Node {
                        label: Some(
                            "label2",
                        ),
                        identifier: "child2@address2",
                        children: [
                            Node {
                                label: Some(
                                    "label21",
                                ),
                                identifier: "child21",
                                children: [],
                                properties: [],
                            },
                        ],
                        properties: [],
                    },
                ],
                properties: [],
            },
        )
        "#
        );
    }

    #[test]
    fn parse_node_with_properties_correctly() {
        assert_debug_snapshot!(
            parse(
                r#"node {
    property-one;
    label1: child1 {
        property-two = "xyg";
    };
};"#
            ),
            @r#"
        Ok(
            Node {
                label: None,
                identifier: "node",
                children: [
                    Node {
                        label: Some(
                            "label1",
                        ),
                        identifier: "child1",
                        children: [],
                        properties: [
                            Property {
                                name: "property-two",
                                value: Values(
                                    PropertyValues(
                                        [
                                            String(
                                                StringValue(
                                                    "xyg",
                                                ),
                                            ),
                                        ],
                                    ),
                                ),
                            },
                        ],
                    },
                ],
                properties: [
                    Property {
                        name: "property-one",
                        value: Bool,
                    },
                ],
            },
        )
        "#
        );
    }

    fn parse(input: &str) -> Result<Node, ParseError> {
        let mut parser = Parser::new(input);
        parse_node(&mut parser)
    }
}
