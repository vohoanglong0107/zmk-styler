use crate::{
    ast::{
        ArrayCell, ArrayValue, BoolPropertyDefinition, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyValue, PropertyValues, StringValue,
    },
    formatter::{Format, Formatter},
};

pub(super) fn format_property(prop: PropertyDefinition, f: &Formatter) -> Format {
    match prop {
        PropertyDefinition::Bool(prop) => format_bool_property(prop, f),
        PropertyDefinition::NonBool(prop) => format_non_bool_property(prop, f),
    }
}

fn format_bool_property(prop: BoolPropertyDefinition, f: &Formatter) -> Format {
    f.pair(f.text(prop.name), f.tag(";"))
}

fn format_non_bool_property(prop: NonBoolPropertyDefinition, f: &Formatter) -> Format {
    f.list([
        f.text(prop.name),
        f.space(),
        f.tag("="),
        f.space(),
        format_property_values(prop.values, f),
        f.tag(";"),
    ])
}

fn format_property_values(values: PropertyValues, f: &Formatter) -> Format {
    f.separated_list(
        values
            .into_iter()
            .map(|value| format_property_value(value, f)),
        f.tag(","),
    )
}

fn format_property_value(value: PropertyValue, f: &Formatter) -> Format {
    match value {
        PropertyValue::Array(array) => format_array(array, f),
        PropertyValue::String(string) => format_string(string, f),
        _ => todo!(),
    }
}

fn format_array(array: ArrayValue, f: &Formatter) -> Format {
    f.list([
        f.tag("<"),
        f.separated_list(
            array.into_iter().map(|cell| format_cell(cell, f)),
            f.space(),
        ),
        f.tag(">"),
    ])
}

fn format_cell(cell: ArrayCell, f: &Formatter) -> Format {
    match cell {
        ArrayCell::Int(int_cell) => f.text(int_cell),
        _ => todo!(),
    }
}

fn format_string(s: StringValue, f: &Formatter) -> Format {
    f.text(s)
}
