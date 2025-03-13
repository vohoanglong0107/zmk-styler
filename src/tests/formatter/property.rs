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
        Concat [
            Concat [],
            Text(/),
            Concat [
                Text( ),
                Text({),
                Concat [
                    Indent(1),
                    Concat [
                        Concat [
                            Text(hold-trigger-on-release),
                            Text(;),
                        ],
                        Concat [],
                    ],
                ],
                Indent(0),
                Text(}),
                Text(;),
            ],
            Concat [],
        ],
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
        Concat [
            Concat [],
            Text(/),
            Concat [
                Text( ),
                Text({),
                Concat [
                    Indent(1),
                    Concat [
                        Concat [
                            Concat [],
                            Text(arr),
                            Text( ),
                            Text(=),
                            Text( ),
                            Concat [
                                Concat [
                                    Text(<),
                                    Concat [
                                        Text(1),
                                        Text( ),
                                        Text(2),
                                        Text( ),
                                        Text(3),
                                    ],
                                    Text(>),
                                ],
                            ],
                            Text(;),
                            Concat [],
                        ],
                        Concat [],
                    ],
                ],
                Indent(0),
                Text(}),
                Text(;),
            ],
            Concat [],
        ],
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
        Concat [
            Concat [],
            Text(/),
            Concat [
                Text( ),
                Text({),
                Concat [
                    Indent(1),
                    Concat [
                        Concat [
                            Concat [],
                            Text(label),
                            Text( ),
                            Text(=),
                            Text( ),
                            Concat [
                                Text("BT_2"),
                            ],
                            Text(;),
                            Concat [],
                        ],
                        Concat [],
                    ],
                ],
                Indent(0),
                Text(}),
                Text(;),
            ],
            Concat [],
        ],
    ]
    "#)
}
