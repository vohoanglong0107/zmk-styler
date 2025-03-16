use crate::{
    ast::{
        ArrayCell, ArrayValue, AstNode, BoolPropertyDefinition, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyValue, PropertyValues, StringValue,
    },
    formatter::{
        rules::{
            format_leading_trivia, format_trailing_trivia, list, nil, pair, separated_list, space,
            tag, text,
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
        format_leading_trivia(f.trivia.leading_trivia(prop.range()), f.source),
        text(&prop.name, f.source),
        space(),
        tag("="),
        space(),
        format_property_values(&prop.values, f),
        tag(";"),
        format_trailing_trivia(f.trivia.trailing_trivia(prop.range()), f.source),
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
