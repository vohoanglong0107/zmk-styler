use insta::{assert_debug_snapshot, assert_snapshot};

use super::{debug_format, debug_formatted};

#[test]
fn format_node_correctly() {
    let test_str = r#"/ {
    a-prop;
    behaviors {
        lower: lower {
            compatible;
            with;
        };
    };
};"#;
    let format = debug_format(test_str);

    let formatted = debug_formatted(test_str);

    assert_snapshot!(formatted, @r"
    / {
        a-prop;
        behaviors {
            lower: lower {
                compatible;
                with;
            };
        };
    };
    ");
    assert_debug_snapshot!(format, @r"
    Concat [
        Concat [
            Text(/),
            Concat [
                Text( ),
                Text({),
                Concat [
                    Indent(1),
                    Concat [
                        Concat [
                            Text(a-prop),
                            Text(;),
                        ],
                        Indent(1),
                        Concat [
                            Text(behaviors),
                            Concat [
                                Text( ),
                                Text({),
                                Concat [
                                    Indent(2),
                                    Concat [
                                        Concat [
                                            Concat [
                                                Text(lower:),
                                                Text( ),
                                            ],
                                            Text(lower),
                                            Concat [
                                                Text( ),
                                                Text({),
                                                Concat [
                                                    Indent(3),
                                                    Concat [
                                                        Concat [
                                                            Text(compatible),
                                                            Text(;),
                                                        ],
                                                        Indent(3),
                                                        Concat [
                                                            Text(with),
                                                            Text(;),
                                                        ],
                                                    ],
                                                ],
                                                Indent(2),
                                                Text(}),
                                                Text(;),
                                            ],
                                        ],
                                    ],
                                ],
                                Indent(1),
                                Text(}),
                                Text(;),
                            ],
                        ],
                    ],
                ],
                Indent(0),
                Text(}),
                Text(;),
            ],
        ],
    ]
    ");
}
