use insta::assert_snapshot;

use super::debug_formatted;

#[test]
fn format_boolean_property() {
    let test_str = "/ {hold-trigger-on-release;};";
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r"
    / {
        hold-trigger-on-release;
    };
    ");
}

#[test]
fn format_i32_array_property() {
    let test_str = "/ {arr = <1 2 3>;};";
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r"
    / {
        arr = <1 2 3>;
    };
    ");
}

#[test]
fn format_string_property() {
    let test_str = "/ {label = \"BT_2\"; };";
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        label = "BT_2";
    };
    "#);
}
