use insta::{assert_debug_snapshot, assert_snapshot};

use super::{debug_format, debug_formatted};

#[test]
fn format_boolean_property() {
    let test_str = "/ {hold-trigger-on-release;};";
    let format = debug_format(test_str);
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r"
    / {
        hold-trigger-on-release;
    };
    ");
    assert_debug_snapshot!(format, @r"
    Concat [
        Text(/),
        Text( ),
        Group [
            Text({),
            Indent(0,Open),
            Text(hold-trigger-on-release),
            Text(;),
            Indent(0,NewLine),
            Indent(0,Close),
            Text(}),
        ],
        Text(;),
    ]
    ")
}

#[test]
fn format_i32_array_property() {
    let test_str = "/ {arr = <1 2 3>;};";
    let format = debug_format(test_str);
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r"
    / {
        arr = <1 2 3>;
    };
    ");
    assert_debug_snapshot!(format, @r"
    Concat [
        Text(/),
        Text( ),
        Group [
            Text({),
            Indent(0,Open),
            Text(arr),
            Text( ),
            Text(=),
            Text( ),
            Text(<),
            Text(1),
            Text( ),
            Text(2),
            Text( ),
            Text(3),
            Text(>),
            Text(;),
            Indent(0,NewLine),
            Indent(0,Close),
            Text(}),
        ],
        Text(;),
    ]
    ")
}

#[test]
fn format_string_property() {
    let test_str = "/ {label = \"BT_2\"; };";
    let format = debug_format(test_str);
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        label = "BT_2";
    };
    "#);
    assert_debug_snapshot!(format, @r#"
    Concat [
        Text(/),
        Text( ),
        Group [
            Text({),
            Indent(0,Open),
            Text(label),
            Text( ),
            Text(=),
            Text( ),
            Text("BT_2"),
            Text(;),
            Indent(0,NewLine),
            Indent(0,Close),
            Text(}),
        ],
        Text(;),
    ]
    "#)
}
