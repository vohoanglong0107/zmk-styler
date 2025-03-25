use insta::assert_snapshot;

use crate::tests::parser::debug_ast;

#[test]
fn parse_node_correctly() {
    assert_snapshot!(debug_ast("node {};"), @r"
    Document@[0..8](
        NodeDefinition@[0..8](
            NonRootNodeIdentifier@[0..4](
                NodeName@[0..4](
                    NAME@[0..4](node)
                )
            )
            NodeBody@[5..8](
                L_CURLY@[5..6]({)
                NodeBodyEntries@[6..6]()
                R_CURLY@[6..7](})
                SEMICOLON@[7..8](;)
            )
        )
    )
    ");
}

#[test]
fn parse_root_node_correctly() {
    assert_snapshot!(
        debug_ast("/ {};"),
        @r"
    Document@[0..5](
        NodeDefinition@[0..5](
            RootNodeIdentifier@[0..1](
                ROOT@[0..1](/)
            )
            NodeBody@[2..5](
                L_CURLY@[2..3]({)
                NodeBodyEntries@[3..3]()
                R_CURLY@[3..4](})
                SEMICOLON@[4..5](;)
            )
        )
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
        NodeDefinition@[0..15](
            Label@[0..6](
                NAME@[0..5](label)
                COLON@[5..6](:)
            )
            NonRootNodeIdentifier@[7..11](
                NodeName@[7..11](
                    NAME@[7..11](node)
                )
            )
            NodeBody@[12..15](
                L_CURLY@[12..13]({)
                NodeBodyEntries@[13..13]()
                R_CURLY@[13..14](})
                SEMICOLON@[14..15](;)
            )
        )
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
        NodeDefinition@[0..18](
            Label@[0..6](
                NAME@[0..5](label)
                COLON@[5..6](:)
            )
            NonRootNodeIdentifier@[7..14](
                NodeName@[7..11](
                    NAME@[7..11](node)
                )
                NodeAddress@[11..14](
                    AT@[11..12](@)
                    INT@[12..14](12)
                )
            )
            NodeBody@[15..18](
                L_CURLY@[15..16]({)
                NodeBodyEntries@[16..16]()
                R_CURLY@[16..17](})
                SEMICOLON@[17..18](;)
            )
        )
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
        NodeDefinition@[0..39](
            NonRootNodeIdentifier@[0..4](
                NodeName@[0..4](
                    NAME@[0..4](node)
                )
            )
            NodeBody@[5..39](
                L_CURLY@[5..6]({)
                NodeBodyEntries@[11..36](
                    NodeDefinition@[11..21](
                        NonRootNodeIdentifier@[11..17](
                            NodeName@[11..17](
                                NAME@[11..17](child1)
                            )
                        )
                        NodeBody@[18..21](
                            L_CURLY@[18..19]({)
                            NodeBodyEntries@[19..19]()
                            R_CURLY@[19..20](})
                            SEMICOLON@[20..21](;)
                        )
                    )
                    NodeDefinition@[26..36](
                        NonRootNodeIdentifier@[26..32](
                            NodeName@[26..32](
                                NAME@[26..32](child2)
                            )
                        )
                        NodeBody@[33..36](
                            L_CURLY@[33..34]({)
                            NodeBodyEntries@[34..34]()
                            R_CURLY@[34..35](})
                            SEMICOLON@[35..36](;)
                        )
                    )
                )
                R_CURLY@[37..38](})
                SEMICOLON@[38..39](;)
            )
        )
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
        NodeDefinition@[0..98](
            NonRootNodeIdentifier@[0..4](
                NodeName@[0..4](
                    NAME@[0..4](node)
                )
            )
            NodeBody@[5..98](
                L_CURLY@[5..6]({)
                NodeBodyEntries@[11..95](
                    NodeDefinition@[11..29](
                        Label@[11..18](
                            NAME@[11..17](label1)
                            COLON@[17..18](:)
                        )
                        NonRootNodeIdentifier@[19..25](
                            NodeName@[19..25](
                                NAME@[19..25](child1)
                            )
                        )
                        NodeBody@[26..29](
                            L_CURLY@[26..27]({)
                            NodeBodyEntries@[27..27]()
                            R_CURLY@[27..28](})
                            SEMICOLON@[28..29](;)
                        )
                    )
                    NodeDefinition@[34..95](
                        Label@[34..41](
                            NAME@[34..40](label2)
                            COLON@[40..41](:)
                        )
                        NonRootNodeIdentifier@[42..57](
                            NodeName@[42..48](
                                NAME@[42..48](child2)
                            )
                            NodeAddress@[48..57](
                                AT@[48..49](@)
                                NAME@[49..57](address2)
                            )
                        )
                        NodeBody@[58..95](
                            L_CURLY@[58..59]({)
                            NodeBodyEntries@[68..88](
                                NodeDefinition@[68..88](
                                    Label@[68..76](
                                        NAME@[68..75](label21)
                                        COLON@[75..76](:)
                                    )
                                    NonRootNodeIdentifier@[77..84](
                                        NodeName@[77..84](
                                            NAME@[77..84](child21)
                                        )
                                    )
                                    NodeBody@[85..88](
                                        L_CURLY@[85..86]({)
                                        NodeBodyEntries@[86..86]()
                                        R_CURLY@[86..87](})
                                        SEMICOLON@[87..88](;)
                                    )
                                )
                            )
                            R_CURLY@[93..94](})
                            SEMICOLON@[94..95](;)
                        )
                    )
                )
                R_CURLY@[96..97](})
                SEMICOLON@[97..98](;)
            )
        )
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
        NodeDefinition@[0..85](
            NonRootNodeIdentifier@[0..4](
                NodeName@[0..4](
                    NAME@[0..4](node)
                )
            )
            NodeBody@[5..85](
                L_CURLY@[5..6]({)
                NodeBodyEntries@[11..82](
                    BoolPropertyDefinition@[11..24](
                        PropertyName@[11..23](
                            NAME@[11..23](property-one)
                        )
                        SEMICOLON@[23..24](;)
                    )
                    NodeDefinition@[29..82](
                        Label@[29..36](
                            NAME@[29..35](label1)
                            COLON@[35..36](:)
                        )
                        NonRootNodeIdentifier@[37..43](
                            NodeName@[37..43](
                                NAME@[37..43](child1)
                            )
                        )
                        NodeBody@[44..82](
                            L_CURLY@[44..45]({)
                            NodeBodyEntries@[54..75](
                                NonBoolPropertyDefinition@[54..75](
                                    PropertyName@[54..66](
                                        NAME@[54..66](property-two)
                                    )
                                    EQUAL@[67..68](=)
                                    PropertyValues@[69..75](
                                        StringValue@[69..74](
                                            STRING@[69..74]("xyg")
                                        )
                                        SEMICOLON@[74..75](;)
                                    )
                                )
                            )
                            R_CURLY@[80..81](})
                            SEMICOLON@[81..82](;)
                        )
                    )
                )
                R_CURLY@[83..84](})
                SEMICOLON@[84..85](;)
            )
        )
    )
    "#
    );
}
