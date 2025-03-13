use crate::{
    ast::{
        ArrayCell, ArrayValue, BoolPropertyDefinition, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyValue, PropertyValues, StringValue,
    },
    formatter::{
        rules::{
            flush_comments_before, format_trailing_comments, list, nil, pair, separated_list,
            space, tag, text,
        },
        Format, FormatContext,
    },
};

pub(super) fn format_property(prop: &PropertyDefinition, f: &mut FormatContext) -> Format {
    match prop {
        PropertyDefinition::Bool(prop) => format_bool_property(prop, f),
        PropertyDefinition::NonBool(prop) => format_non_bool_property(prop, f),
    }
}

fn format_bool_property(prop: &BoolPropertyDefinition, f: &FormatContext) -> Format {
    pair(text(&prop.name, f.source), tag(";"))
}

fn format_non_bool_property(prop: &NonBoolPropertyDefinition, f: &mut FormatContext) -> Format {
    list([
        flush_comments_before(prop, f.source, &mut f.trivia),
        text(&prop.name, f.source),
        space(),
        tag("="),
        space(),
        format_property_values(&prop.values, f),
        tag(";"),
        format_trailing_comments(prop, f.source, &mut f.trivia),
    ])
}

fn format_property_values(values: &PropertyValues, f: &FormatContext) -> Format {
    separated_list(
        values
            .values
            .iter()
            .map(|value| format_property_value(value, f)),
        tag(","),
    )
}

fn format_property_value(value: &PropertyValue, f: &FormatContext) -> Format {
    match value {
        PropertyValue::Array(array) => format_array(array, f),
        PropertyValue::String(string) => format_string(string, f),
        _ => todo!(),
    }
}

fn format_array(array: &ArrayValue, f: &FormatContext) -> Format {
    list([
        tag("<"),
        separated_list(array.cells.iter().map(|cell| format_cell(cell, f)), space()),
        tag(">"),
    ])
}

fn format_cell(cell: &ArrayCell, f: &FormatContext) -> Format {
    match cell {
        ArrayCell::Int(int_cell) => text(int_cell, f.source),
        _ => todo!(),
    }
}

fn format_string(s: &StringValue, f: &FormatContext) -> Format {
    text(s, f.source)
}
