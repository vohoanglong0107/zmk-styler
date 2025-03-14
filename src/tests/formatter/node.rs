use insta::assert_snapshot;

use super::debug_formatted;

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
}
