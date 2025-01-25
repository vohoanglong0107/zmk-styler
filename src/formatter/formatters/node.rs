use crate::{
    ast::{Node, Property},
    formatter::ir::{concat, indent, new_line, nil, space, tag, text, Document},
};

use super::property::format_property;

pub(crate) fn format_node(node: Node) -> Document {
    concat(vec![
        format_label(node.label),
        text(node.name),
        format_address(node.address),
        format_node_body(node.properties, node.children),
    ])
}

fn format_label(label: Option<String>) -> Document {
    match label {
        Some(label) => concat(vec![text(label), tag(":"), space()]),
        None => nil(),
    }
}

fn format_address(address: Option<String>) -> Document {
    match address {
        Some(address) => concat(vec![tag("@"), text(address)]),
        None => nil(),
    }
}

fn format_node_body(properties: Vec<Property>, children: Vec<Node>) -> Document {
    let multiline = !properties.is_empty() || !children.is_empty();

    concat(vec![
        space(),
        tag("{"),
        format_properties(properties),
        format_children(children),
        if multiline { new_line() } else { nil() },
        tag("}"),
        tag(";"),
    ])
}

fn format_properties(properties: Vec<Property>) -> Document {
    if properties.is_empty() {
        return nil();
    }
    let props = properties
        .into_iter()
        .flat_map(|prop| [new_line(), format_property(prop)])
        .collect();
    indent(concat(props))
}

fn format_children(nodes: Vec<Node>) -> Document {
    if nodes.is_empty() {
        return nil();
    }
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
                Nil,
                Text(
                    "/",
                ),
                Nil,
                Concat(
                    [
                        Text(
                            " ",
                        ),
                        Text(
                            "{",
                        ),
                        Concat(
                            [
                                Indent {
                                    level: 1,
                                },
                                Concat(
                                    [
                                        Text(
                                            "a-prop",
                                        ),
                                        Text(
                                            ";",
                                        ),
                                    ],
                                ),
                            ],
                        ),
                        Concat(
                            [
                                Indent {
                                    level: 1,
                                },
                                Concat(
                                    [
                                        Nil,
                                        Text(
                                            "behaviors",
                                        ),
                                        Nil,
                                        Concat(
                                            [
                                                Text(
                                                    " ",
                                                ),
                                                Text(
                                                    "{",
                                                ),
                                                Nil,
                                                Concat(
                                                    [
                                                        Indent {
                                                            level: 2,
                                                        },
                                                        Concat(
                                                            [
                                                                Concat(
                                                                    [
                                                                        Text(
                                                                            "lower",
                                                                        ),
                                                                        Text(
                                                                            ":",
                                                                        ),
                                                                        Text(
                                                                            " ",
                                                                        ),
                                                                    ],
                                                                ),
                                                                Text(
                                                                    "lower",
                                                                ),
                                                                Nil,
                                                                Concat(
                                                                    [
                                                                        Text(
                                                                            " ",
                                                                        ),
                                                                        Text(
                                                                            "{",
                                                                        ),
                                                                        Concat(
                                                                            [
                                                                                Indent {
                                                                                    level: 3,
                                                                                },
                                                                                Concat(
                                                                                    [
                                                                                        Text(
                                                                                            "compatible",
                                                                                        ),
                                                                                        Text(
                                                                                            ";",
                                                                                        ),
                                                                                    ],
                                                                                ),
                                                                                Indent {
                                                                                    level: 3,
                                                                                },
                                                                                Concat(
                                                                                    [
                                                                                        Text(
                                                                                            "with",
                                                                                        ),
                                                                                        Text(
                                                                                            ";",
                                                                                        ),
                                                                                    ],
                                                                                ),
                                                                            ],
                                                                        ),
                                                                        Nil,
                                                                        Indent {
                                                                            level: 2,
                                                                        },
                                                                        Text(
                                                                            "}",
                                                                        ),
                                                                        Text(
                                                                            ";",
                                                                        ),
                                                                    ],
                                                                ),
                                                            ],
                                                        ),
                                                    ],
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
                                            ],
                                        ),
                                    ],
                                ),
                            ],
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
