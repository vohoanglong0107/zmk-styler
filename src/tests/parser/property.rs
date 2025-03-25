use insta::assert_snapshot;

use crate::tests::parser::debug_ast;

#[test]
fn parse_boolean_property_correctly() {
    assert_snapshot!(
        debug_ast("/ {hold-trigger-on-release;};"),
        @r"
    Document@[0..29](
        NodeDefinition@[0..29](
            RootNodeIdentifier@[0..1](
                ROOT@[0..1](/)
            )
            NodeBody@[2..29](
                L_CURLY@[2..3]({)
                NodeBodyEntries@[3..27](
                    BoolPropertyDefinition@[3..27](
                        PropertyName@[3..26](
                            NAME@[3..26](hold-trigger-on-release)
                        )
                        SEMICOLON@[26..27](;)
                    )
                )
                R_CURLY@[27..28](})
                SEMICOLON@[28..29](;)
            )
        )
    )
    "
    )
}

#[test]
fn parse_i32_array_property_correctly() {
    assert_snapshot!(
        debug_ast("/ {an-array = <0 1 2 3>;};"),
        @r"
    Document@[0..26](
        NodeDefinition@[0..26](
            RootNodeIdentifier@[0..1](
                ROOT@[0..1](/)
            )
            NodeBody@[2..26](
                L_CURLY@[2..3]({)
                NodeBodyEntries@[3..24](
                    NonBoolPropertyDefinition@[3..24](
                        PropertyName@[3..11](
                            NAME@[3..11](an-array)
                        )
                        EQUAL@[12..13](=)
                        PropertyValues@[14..24](
                            ArrayValue@[14..23](
                                L_ANGLE@[14..15](<)
                                IntCell@[15..16](
                                    INT@[15..16](0)
                                )
                                IntCell@[17..18](
                                    INT@[17..18](1)
                                )
                                IntCell@[19..20](
                                    INT@[19..20](2)
                                )
                                IntCell@[21..22](
                                    INT@[21..22](3)
                                )
                                R_ANGLE@[22..23](>)
                            )
                            SEMICOLON@[23..24](;)
                        )
                    )
                )
                R_CURLY@[24..25](})
                SEMICOLON@[25..26](;)
            )
        )
    )
    "
    )
}

#[test]
fn parse_string_property_correctly() {
    assert_snapshot!(
        debug_ast(r#"/ {compatible = "zmk,behavior-tap-dance";};"#),
        @r#"
    Document@[0..43](
        NodeDefinition@[0..43](
            RootNodeIdentifier@[0..1](
                ROOT@[0..1](/)
            )
            NodeBody@[2..43](
                L_CURLY@[2..3]({)
                NodeBodyEntries@[3..41](
                    NonBoolPropertyDefinition@[3..41](
                        PropertyName@[3..13](
                            NAME@[3..13](compatible)
                        )
                        EQUAL@[14..15](=)
                        PropertyValues@[16..41](
                            StringValue@[16..40](
                                STRING@[16..40]("zmk,behavior-tap-dance")
                            )
                            SEMICOLON@[40..41](;)
                        )
                    )
                )
                R_CURLY@[41..42](})
                SEMICOLON@[42..43](;)
            )
        )
    )
    "#
    )
}

#[test]
fn parse_ill_formed_string_property() {
    assert_snapshot!(
        debug_ast(r#"/ {"zmk,behavior-tap-dance";"#),
        @r#"
    error: Expected R_CURLY, but found EOF
      |
    1 | / {"zmk,behavior-tap-dance";
      |                             ^ Expected R_CURLY, but found EOF
      |

    error: Expected SEMICOLON, but found EOF
      |
    1 | / {"zmk,behavior-tap-dance";
      |                             ^ Expected SEMICOLON, but found EOF
      |

    Document@[0..28](
        NodeDefinition@[0..28](
            RootNodeIdentifier@[0..1](
                ROOT@[0..1](/)
            )
            NodeBody@[2..28](
                L_CURLY@[2..3]({)
                NodeBodyEntries@[3..28](
                    STRING@[3..27]("zmk,behavior-tap-dance")
                    SEMICOLON@[27..28](;)
                )
            )
        )
    )
    "#
    )
}
