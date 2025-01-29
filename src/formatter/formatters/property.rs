use itertools::Itertools;

use crate::{
    ast::{ArrayCell, ArrayValue, NonBoolPropertyValue, Property, PropertyValue, PropertyValues},
    formatter::ir::{concat, nil, space, tag, text, Document},
};

pub(super) fn format_property(prop: Property) -> Document {
    let value = match prop.value {
        PropertyValue::Bool => nil(),
        PropertyValue::Values(values) => {
            concat([space(), text("="), space(), format_property_values(values)])
        }
    };
    concat([text(prop.name), value, tag(";")])
}

fn format_property_values(values: PropertyValues) -> Document {
    let formatted_values =
        Itertools::intersperse(values.into_iter().map(format_property_value), tag(","));
    concat(formatted_values)
}

fn format_property_value(value: NonBoolPropertyValue) -> Document {
    match value {
        NonBoolPropertyValue::Array(array) => format_array(array),
        _ => todo!(),
    }
}

fn format_array(array: ArrayValue) -> Document {
    let mut formatted_array = vec![tag("<")];
    let formatted_array_elements =
        Itertools::intersperse(array.into_iter().map(format_cell), space());
    formatted_array.extend(formatted_array_elements);
    formatted_array.push(tag(">"));
    concat(formatted_array)
}

fn format_cell(cell: ArrayCell) -> Document {
    match cell {
        ArrayCell::Int(int_cell) => text(int_cell),
        _ => todo!(),
    }
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

    #[test]
    fn format_i32_array_property() {
        let (_, prop) = parse_property("arr = <1 2 3>;").unwrap();
        assert_debug_snapshot!(format_property(prop), @r#"
        Concat(
            [
                Text(
                    "arr",
                ),
                Concat(
                    [
                        Text(
                            " ",
                        ),
                        Text(
                            "=",
                        ),
                        Text(
                            " ",
                        ),
                        Concat(
                            [
                                Concat(
                                    [
                                        Text(
                                            "<",
                                        ),
                                        Text(
                                            "1",
                                        ),
                                        Text(
                                            " ",
                                        ),
                                        Text(
                                            "2",
                                        ),
                                        Text(
                                            " ",
                                        ),
                                        Text(
                                            "3",
                                        ),
                                        Text(
                                            ">",
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
                Text(
                    ";",
                ),
            ],
        )
        "#)
    }
}
