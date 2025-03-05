use insta::assert_snapshot;

use crate::tests::parser::debug_ast;

#[test]
fn parse_boolean_property_correctly() {
    assert_snapshot!(
        debug_ast("/ {hold-trigger-on-release;};"),
        @r"
    Document@[0..29](
        Node@[0..29](
            Identifier@[0..1](/),
            NodeBody@[2..29](
                BoolProperty@[3..27](
                    PropertyName@[3..26](hold-trigger-on-release),
                ),
            ),
        ),
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
        Node@[0..26](
            Identifier@[0..1](/),
            NodeBody@[2..26](
                NonBoolProperty@[3..24](
                    PropertyName@[3..11](an-array),
                    PropertyValues@[14..24](
                        Array@[14..23](
                            Int@[15..16](0),
                            Int@[17..18](1),
                            Int@[19..20](2),
                            Int@[21..22](3),
                        ),
                    ),
                ),
            ),
        ),
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
        Node@[0..43](
            Identifier@[0..1](/),
            NodeBody@[2..43](
                NonBoolProperty@[3..41](
                    PropertyName@[3..13](compatible),
                    PropertyValues@[16..41](
                        String@[16..40]("zmk,behavior-tap-dance"),
                    ),
                ),
            ),
        ),
    )
    "#
    )
}
