use insta::assert_snapshot;

use super::debug_formatted;

#[test]
fn format_single_line_comments() {
    let test_str = "/ {
        // This is a label
        label = \"BT_2\"; // End of label
        // This is definitely the end of label

        // This is start of node1

        // This
        // is node1
        node1 {
            // This
            // is
            // node2
            node2 {}; // End of node 2
            // End
        // of
        // node 1
        };
    };";
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        // This is a label
        label = "BT_2"; // End of label
        // This is definitely the end of label

        // This is start of node1

        // This
        // is node1
        node1 {
            // This
            // is
            // node2
            node2 {}; // End of node 2
            // End
            // of
            // node 1
        };
    };
    "#);
}

#[test]
fn format_block_comments() {
    let test_str = r#"/ {
        /* This is a label */label = "BT_2"; /* End of label */

        /* This is
         * node 1*/ node1 {
            /* This
             * is
             * node2*/
            node2 {}; /* End */ /* of */ /* node2 */
            node3 {}; /* End
            of */ /* of */ /* node3 */
            /* End
             * Of
             * Node1
             */
        };
    };"#;
    let formatted = debug_formatted(test_str);
    assert_snapshot!(formatted, @r#"
    / {
        /* This is a label */ label = "BT_2"; /* End of label */

        /* This is
         * node 1*/
        node1 {
            /* This
             * is
             * node2*/
            node2 {}; /* End */ /* of */ /* node2 */
            node3 {};
            /* End
             of */
            /* of */
            /* node3 */
            /* End
             * Of
             * Node1
             */
        };
    };
    "#);
}
