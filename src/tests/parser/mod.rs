mod node;
mod property;

use crate::{
    ast::{
        ArrayCell, ArrayValue, AstNode, BoolPropertyDefinition, Document, Identifier, IntValue,
        Label, NodeBody, NodeBodyEntry, NodeDefinition, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyName, PropertyValue, PropertyValues, Statement, StringValue,
    },
    formatter::{
        rules::{indent, list, new_line, nil, pair, separated_list, tag, text},
        Format, FormatContext, Writer,
    },
    parser::parse,
    source::Source,
};

fn debug_ast(test_str: &str) -> String {
    let source = Source::new(test_str);
    match parse(&source) {
        Ok((doc, comments)) => {
            let formatter = FormatContext::new(&source, comments);
            let mut writer = Writer::default();
            writer.write(serialize_doc(doc, &formatter));
            writer.finish()
        }
        Err(e) => e.to_string(),
    }
}

fn serialize_doc(doc: Document, f: &FormatContext) -> Format {
    list([
        tag("Document@"),
        tag(doc.range()),
        tag("("),
        indent(serialize_statements(doc.statements, f)),
        new_line(),
        tag(")"),
    ])
}

fn serialize_statements(statements: Vec<Statement>, f: &FormatContext) -> Format {
    separated_list(
        statements.into_iter().map(|s| serialize_statement(s, f)),
        new_line(),
    )
}

fn serialize_statement(statement: Statement, f: &FormatContext) -> Format {
    match statement {
        Statement::Node(node) => serialize_node(node, f),
    }
}

fn serialize_node(node: NodeDefinition, f: &FormatContext) -> Format {
    list([
        tag("Node@"),
        tag(node.range),
        tag("("),
        indent(list([
            node.label.map_or(nil(), |label| serialize_label(label, f)),
            serialize_node_identifier(node.identifier, f),
            serialize_node_body(node.body, f),
        ])),
        new_line(),
        tag(")"),
        tag(","),
    ])
}

fn serialize_label(label: Label, f: &FormatContext) -> Format {
    list([
        tag("Label@"),
        tag(label.range()),
        tag("("),
        text(label, f.source),
        tag(")"),
        tag(","),
        new_line(),
    ])
}

fn serialize_node_identifier(identifier: Identifier, f: &FormatContext) -> Format {
    list([
        tag("Identifier@"),
        tag(identifier.range()),
        tag("("),
        text(identifier, f.source),
        tag(")"),
        tag(","),
        new_line(),
    ])
}

fn serialize_node_body(node_body: NodeBody, f: &FormatContext) -> Format {
    list([
        tag("NodeBody@"),
        tag(node_body.range()),
        tag("("),
        if node_body.entries.is_empty() {
            text(node_body, f.source)
        } else {
            pair(
                indent(serialize_node_body_entries(node_body.entries, f)),
                new_line(),
            )
        },
        tag(")"),
        tag(","),
    ])
}

fn serialize_node_body_entries(entries: Vec<NodeBodyEntry>, f: &FormatContext) -> Format {
    separated_list(
        entries
            .into_iter()
            .map(|entry| serialize_node_body_entry(entry, f)),
        new_line(),
    )
}

fn serialize_node_body_entry(entry: NodeBodyEntry, f: &FormatContext) -> Format {
    match entry {
        NodeBodyEntry::Node(node) => serialize_node(node, f),
        NodeBodyEntry::Property(prop) => serialize_property(prop, f),
    }
}

fn serialize_property(property: PropertyDefinition, f: &FormatContext) -> Format {
    match property {
        PropertyDefinition::Bool(prop) => serialize_bool_property(prop, f),
        PropertyDefinition::NonBool(prop) => serialize_non_bool_property(prop, f),
    }
}

fn serialize_bool_property(property: BoolPropertyDefinition, f: &FormatContext) -> Format {
    list([
        tag("BoolProperty@"),
        tag(property.range()),
        tag("("),
        indent(serialize_property_name(property.name, f)),
        new_line(),
        tag(")"),
        tag(","),
    ])
}

fn serialize_non_bool_property(property: NonBoolPropertyDefinition, f: &FormatContext) -> Format {
    list([
        tag("NonBoolProperty@"),
        tag(property.range()),
        tag("("),
        indent(list([
            serialize_property_name(property.name, f),
            new_line(),
            serialize_property_values(property.values, f),
        ])),
        new_line(),
        tag(")"),
        tag(","),
    ])
}

fn serialize_property_name(property: PropertyName, f: &FormatContext) -> Format {
    list([
        tag("PropertyName@"),
        tag(property.range()),
        tag("("),
        text(property, f.source),
        tag(")"),
        tag(","),
    ])
}

fn serialize_property_values(property_values: PropertyValues, f: &FormatContext) -> Format {
    list([
        tag("PropertyValues@"),
        tag(property_values.range()),
        tag("("),
        indent(separated_list(
            property_values
                .into_iter()
                .map(|value| serialize_property_value(value, f)),
            new_line(),
        )),
        new_line(),
        tag(")"),
        tag(","),
    ])
}

fn serialize_property_value(property_value: PropertyValue, f: &FormatContext) -> Format {
    match property_value {
        PropertyValue::Array(array) => serialize_array(array, f),
        PropertyValue::String(s) => serialize_string(s, f),
        _ => todo!(),
    }
}

fn serialize_array(array: ArrayValue, f: &FormatContext) -> Format {
    list([
        tag("Array@"),
        tag(array.range()),
        tag("("),
        indent(separated_list(
            array
                .into_iter()
                .map(|value| serialize_array_cell(value, f)),
            new_line(),
        )),
        new_line(),
        tag(")"),
        tag(","),
    ])
}

fn serialize_array_cell(cell: ArrayCell, f: &FormatContext) -> Format {
    match cell {
        ArrayCell::Int(int_value) => serialize_int_value(int_value, f),
        _ => todo!(),
    }
}

fn serialize_int_value(cell: IntValue, f: &FormatContext) -> Format {
    list([
        tag("Int@"),
        tag(cell.range()),
        tag("("),
        text(cell, f.source),
        tag(")"),
        tag(","),
    ])
}

fn serialize_string(s: StringValue, f: &FormatContext) -> Format {
    list([
        tag("String@"),
        tag(s.range()),
        tag("("),
        text(s, f.source),
        tag(")"),
        tag(","),
    ])
}
