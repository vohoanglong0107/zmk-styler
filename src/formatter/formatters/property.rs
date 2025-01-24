use crate::{
    ast::Property,
    formatter::ir::{concat, tag, text, Document},
};

pub(super) fn format_property(prop: Property) -> Document {
    concat(vec![text(prop.name), tag(";")])
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::parser::property::parse_property;

    use super::*;
    #[test]
    fn format_boolean_property() {
        let (_, prop) = parse_property("hold-trigger-on-release;").unwrap();
        assert_debug_snapshot!(format_property(prop), @r#"
        Concat(
            [
                Text(
                    "hold-trigger-on-release",
                ),
                Text(
                    ";",
                ),
            ],
        )
        "#)
    }
}
