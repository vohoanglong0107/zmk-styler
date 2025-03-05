use insta::assert_snapshot;

use crate::tests::parser::debug_ast;

#[test]
fn parse_node_correctly() {
    assert_snapshot!(debug_ast("node {};"), @r"
    Document@[0..8](
        Node@[0..8](
            Identifier@[0..4](node),
            NodeBody@[5..8]({};),
        ),
    )
    ");
}

#[test]
fn parse_root_node_correctly() {
    assert_snapshot!(
        debug_ast("/ {};"),
        @r"
    Document@[0..5](
        Node@[0..5](
            Identifier@[0..1](/),
            NodeBody@[2..5]({};),
        ),
    )
    "
    );
}

#[test]
fn parse_node_with_label_correctly() {
    assert_snapshot!(
        debug_ast("label: node {};"),
        @r"
    Document@[0..15](
        Node@[0..15](
            Label@[0..6](label:),
            Identifier@[7..11](node),
            NodeBody@[12..15]({};),
        ),
    )
    "
    );
}

#[test]
fn parse_node_with_address_correctly() {
    assert_snapshot!(
        debug_ast("label: node@12 {};"),
        @r"
    Document@[0..18](
        Node@[0..18](
            Label@[0..6](label:),
            Identifier@[7..14](node@12),
            NodeBody@[15..18]({};),
        ),
    )
    "
    );
}

#[test]
fn parse_node_with_children_correctly() {
    assert_snapshot!(
        debug_ast(
            r#"node {
    child1 {};
    child2 {};
};"#
        ),
        @r"
    Document@[0..39](
        Node@[0..39](
            Identifier@[0..4](node),
            NodeBody@[5..39](
                Node@[11..21](
                    Identifier@[11..17](child1),
                    NodeBody@[18..21]({};),
                ),
                Node@[26..36](
                    Identifier@[26..32](child2),
                    NodeBody@[33..36]({};),
                ),
            ),
        ),
    )
    "
    );
}

#[test]
fn parse_node_with_nested_children_correctly() {
    assert_snapshot!(
        debug_ast(
            r#"node {
    label1: child1 {};
    label2: child2@address2 {
        label21: child21 {};
    };
};"#
        ),
        @r"
    Document@[0..98](
        Node@[0..98](
            Identifier@[0..4](node),
            NodeBody@[5..98](
                Node@[11..29](
                    Label@[11..18](label1:),
                    Identifier@[19..25](child1),
                    NodeBody@[26..29]({};),
                ),
                Node@[34..95](
                    Label@[34..41](label2:),
                    Identifier@[42..57](child2@address2),
                    NodeBody@[58..95](
                        Node@[68..88](
                            Label@[68..76](label21:),
                            Identifier@[77..84](child21),
                            NodeBody@[85..88]({};),
                        ),
                    ),
                ),
            ),
        ),
    )
    "
    );
}

#[test]
fn parse_node_with_properties_correctly() {
    assert_snapshot!(
        debug_ast(
            r#"node {
    property-one;
    label1: child1 {
        property-two = "xyg";
    };
};"#
        ),
        @r#"
    Document@[0..85](
        Node@[0..85](
            Identifier@[0..4](node),
            NodeBody@[5..85](
                BoolProperty@[11..24](
                    PropertyName@[11..23](property-one),
                ),
                Node@[29..82](
                    Label@[29..36](label1:),
                    Identifier@[37..43](child1),
                    NodeBody@[44..82](
                        NonBoolProperty@[54..75](
                            PropertyName@[54..66](property-two),
                            PropertyValues@[69..75](
                                String@[69..74]("xyg"),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
    "#
    );
}
