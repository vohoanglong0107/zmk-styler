use insta::{assert_debug_snapshot, assert_snapshot};

use super::{debug_format, debug_formatted};

#[test]
fn format_single_line_comments() {
    let test_str = "/ {
        // This is a label
        label = \"BT_2\"; 

        // This
        // is node1
        node1 {
            // This
            // is
            // node2
            node2 {};
        };
    };";
    let format = debug_format(test_str);
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        // This is a label
        label = "BT_2";
        // This
        // is node1
        node1 {
            // This
            // is
            // node2
            node2 {};
        };
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
                            Concat [
                                Concat [
                                    Text(// This is a label),
                                    Indent(1),
                                ],
                            ],
                            Text(label),
                            Text( ),
                            Text(=),
                            Text( ),
                            Concat [
                                Text("BT_2"),
                            ],
                            Text(;),
                        ],
                        Indent(1),
                        Concat [
                            Concat [
                                Concat [
                                    Text(// This),
                                    Indent(1),
                                ],
                                Concat [
                                    Text(// is node1),
                                    Indent(1),
                                ],
                            ],
                            Text(node1),
                            Concat [
                                Text( ),
                                Text({),
                                Concat [
                                    Indent(2),
                                    Concat [
                                        Concat [
                                            Concat [
                                                Concat [
                                                    Text(// This),
                                                    Indent(2),
                                                ],
                                                Concat [
                                                    Text(// is),
                                                    Indent(2),
                                                ],
                                                Concat [
                                                    Text(// node2),
                                                    Indent(2),
                                                ],
                                            ],
                                            Text(node2),
                                            Concat [
                                                Text( ),
                                                Text({),
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
    "#)
}

#[test]
fn format_block_comments() {
    let test_str = r#"/ {
        /* This is a label */label = "BT_2"; 

        /* This is
         * node 1*/
        node1 {
            /* This
             * is
             * node2*/
            node2 {};
        };
    };"#;
    let format = debug_format(test_str);
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        /* This is a label */ label = "BT_2";
        /* This is
         * node 1*/
        node1 {
            /* This
             * is
             * node2*/
            node2 {};
        };
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
                            Concat [
                                Concat [
                                    Text(/* This is a label */),
                                    Text( ),
                                ],
                            ],
                            Text(label),
                            Text( ),
                            Text(=),
                            Text( ),
                            Concat [
                                Text("BT_2"),
                            ],
                            Text(;),
                        ],
                        Indent(1),
                        Concat [
                            Concat [
                                Concat [
                                    Text(/* This is),
                                    Indent(1),
                                    Text( * node 1*/),
                                    Indent(1),
                                ],
                            ],
                            Text(node1),
                            Concat [
                                Text( ),
                                Text({),
                                Concat [
                                    Indent(2),
                                    Concat [
                                        Concat [
                                            Concat [
                                                Concat [
                                                    Text(/* This),
                                                    Indent(2),
                                                    Text( * is),
                                                    Indent(2),
                                                    Text( * node2*/),
                                                    Indent(2),
                                                ],
                                            ],
                                            Text(node2),
                                            Concat [
                                                Text( ),
                                                Text({),
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
    "#)
}
