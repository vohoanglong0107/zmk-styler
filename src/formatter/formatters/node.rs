use crate::{
    ast::{Node, Property},
    formatter::ir::{concat, indent, new_line, space, tag, text, Document},
};

use super::property::format_property;

pub(crate) fn format_node(node: Node) -> Document {
    let mut docs = Vec::new();
    if let Some(label) = node.label {
        docs.push(format_label(label));
    }
    docs.push(text(node.name));
    if let Some(address) = node.address {
        docs.push(format_address(address));
    }
    docs.push(format_node_body(node.properties, node.children));
    concat(docs)
}

fn format_label(label: String) -> Document {
    concat(vec![text(label), tag(":"), space()])
}

fn format_address(address: String) -> Document {
    concat(vec![tag("@"), text(address)])
}

fn format_node_body(properties: Vec<Property>, children: Vec<Node>) -> Document {
    let mut docs = Vec::new();

    docs.extend([space(), tag("{")]);
    let should_new_line_closing_bracket = !properties.is_empty() || !children.is_empty();
    if !properties.is_empty() {
        docs.push(format_properties(properties));
    }
    if !children.is_empty() {
        docs.push(format_children(children));
    }

    if should_new_line_closing_bracket {
        docs.push(new_line());
    }
    docs.extend([tag("}"), tag(";")]);
    concat(docs)
}

fn format_properties(properties: Vec<Property>) -> Document {
    let mut props = Vec::new();
    props.extend(
        properties
            .into_iter()
            .flat_map(|prop| vec![new_line(), format_property(prop)]),
    );
    indent(concat(props))
}

fn format_children(nodes: Vec<Node>) -> Document {
    let mut children = Vec::new();
    children.extend(
        nodes
            .into_iter()
            .flat_map(|child| vec![new_line(), format_node(child)]),
    );
    indent(concat(children))
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::parser::node::parse_node;

    use super::*;
    #[test]
    fn format_node_correctly() {
        let node = unwraped_parse_node(
            r#"/ {
    a-prop;
    behaviors {
        lower: lower {
            compatible;
            with;
        };
    };
};"#,
        );
        assert_debug_snapshot!(format_node(node), @r#"
        Concat(
            [
                Text(
                    "/",
                ),
                Text(
                    " ",
                ),
                Text(
                    "{",
                ),
                Indent {
                    level: 1,
                },
                Text(
                    "a-prop",
                ),
                Text(
                    ";",
                ),
                Indent {
                    level: 1,
                },
                Text(
                    "behaviors",
                ),
                Text(
                    " ",
                ),
                Text(
                    "{",
                ),
                Indent {
                    level: 2,
                },
                Text(
                    "lower",
                ),
                Text(
                    ":",
                ),
                Text(
                    " ",
                ),
                Text(
                    "lower",
                ),
                Text(
                    " ",
                ),
                Text(
                    "{",
                ),
                Indent {
                    level: 3,
                },
                Text(
                    "compatible",
                ),
                Text(
                    ";",
                ),
                Indent {
                    level: 3,
                },
                Text(
                    "with",
                ),
                Text(
                    ";",
                ),
                Indent {
                    level: 2,
                },
                Text(
                    "}",
                ),
                Text(
                    ";",
                ),
                Indent {
                    level: 1,
                },
                Text(
                    "}",
                ),
                Text(
                    ";",
                ),
                Indent {
                    level: 0,
                },
                Text(
                    "}",
                ),
                Text(
                    ";",
                ),
            ],
        )
        "#);
    }

    fn unwraped_parse_node(input: &str) -> Node {
        let (_, node) = parse_node(input).unwrap();
        node
    }
}
