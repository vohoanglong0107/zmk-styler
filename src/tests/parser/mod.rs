mod node;
mod property;

use crate::{
    ast::{
        ArrayCell, ArrayValue, AstNode, BoolPropertyDefinition, Document, Identifier, IntValue,
        Label, NodeBody, NodeBodyEntry, NodeDefinition, NonBoolPropertyDefinition,
        PropertyDefinition, PropertyName, PropertyValue, PropertyValues, Statement, StringValue,
    },
    formatter::{Format, Formatter, Writer},
    parser::parse,
    source::Source,
};

fn debug_ast(test_str: &str) -> String {
    let source = Source::new(test_str);
    let formatter = Formatter::new(&source);
    match parse(&source) {
        Ok(doc) => {
            let mut writer = Writer::default();
            writer.write(serialize_doc(doc, &formatter));
            writer.finish()
        }
        Err(e) => e.to_string(),
    }
}

fn serialize_doc(doc: Document, f: &Formatter) -> Format {
    f.list([
        f.tag("Document@"),
        f.tag(doc.range()),
        f.tag("("),
        f.indent(serialize_statements(doc.statements, f)),
        f.new_line(),
        f.tag(")"),
    ])
}

fn serialize_statements(statements: Vec<Statement>, f: &Formatter) -> Format {
    f.separated_list(
        statements.into_iter().map(|s| serialize_statement(s, f)),
        f.new_line(),
    )
}

fn serialize_statement(statement: Statement, f: &Formatter) -> Format {
    match statement {
        Statement::Node(node) => serialize_node(node, f),
    }
}

fn serialize_node(node: NodeDefinition, f: &Formatter) -> Format {
    f.list([
        f.tag("Node@"),
        f.tag(node.range),
        f.tag("("),
        f.indent(
            f.list([
                node.label
                    .map_or(f.nil(), |label| serialize_label(label, f)),
                serialize_node_identifier(node.identifier, f),
                serialize_node_body(node.body, f),
            ]),
        ),
        f.new_line(),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_label(label: Label, f: &Formatter) -> Format {
    f.list([
        f.tag("Label@"),
        f.tag(label.range()),
        f.tag("("),
        f.text(label),
        f.tag(")"),
        f.tag(","),
        f.new_line(),
    ])
}

fn serialize_node_identifier(identifier: Identifier, f: &Formatter) -> Format {
    f.list([
        f.tag("Identifier@"),
        f.tag(identifier.range()),
        f.tag("("),
        f.text(identifier),
        f.tag(")"),
        f.tag(","),
        f.new_line(),
    ])
}

fn serialize_node_body(node_body: NodeBody, f: &Formatter) -> Format {
    f.list([
        f.tag("NodeBody@"),
        f.tag(node_body.range()),
        f.tag("("),
        if node_body.entries.is_empty() {
            f.text(node_body)
        } else {
            f.pair(
                f.indent(serialize_node_body_entries(node_body.entries, f)),
                f.new_line(),
            )
        },
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_node_body_entries(entries: Vec<NodeBodyEntry>, f: &Formatter) -> Format {
    f.separated_list(
        entries
            .into_iter()
            .map(|entry| serialize_node_body_entry(entry, f)),
        f.new_line(),
    )
}

fn serialize_node_body_entry(entry: NodeBodyEntry, f: &Formatter) -> Format {
    match entry {
        NodeBodyEntry::Node(node) => serialize_node(node, f),
        NodeBodyEntry::Property(prop) => serialize_property(prop, f),
    }
}

fn serialize_property(property: PropertyDefinition, f: &Formatter) -> Format {
    match property {
        PropertyDefinition::Bool(prop) => serialize_bool_property(prop, f),
        PropertyDefinition::NonBool(prop) => serialize_non_bool_property(prop, f),
    }
}

fn serialize_bool_property(property: BoolPropertyDefinition, f: &Formatter) -> Format {
    f.list([
        f.tag("BoolProperty@"),
        f.tag(property.range()),
        f.tag("("),
        f.indent(serialize_property_name(property.name, f)),
        f.new_line(),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_non_bool_property(property: NonBoolPropertyDefinition, f: &Formatter) -> Format {
    f.list([
        f.tag("NonBoolProperty@"),
        f.tag(property.range()),
        f.tag("("),
        f.indent(f.list([
            serialize_property_name(property.name, f),
            f.new_line(),
            serialize_property_values(property.values, f),
        ])),
        f.new_line(),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_property_name(property: PropertyName, f: &Formatter) -> Format {
    f.list([
        f.tag("PropertyName@"),
        f.tag(property.range()),
        f.tag("("),
        f.text(property),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_property_values(property_values: PropertyValues, f: &Formatter) -> Format {
    f.list([
        f.tag("PropertyValues@"),
        f.tag(property_values.range()),
        f.tag("("),
        f.indent(
            f.separated_list(
                property_values
                    .into_iter()
                    .map(|value| serialize_property_value(value, f)),
                f.new_line(),
            ),
        ),
        f.new_line(),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_property_value(property_value: PropertyValue, f: &Formatter) -> Format {
    match property_value {
        PropertyValue::Array(array) => serialize_array(array, f),
        PropertyValue::String(s) => serialize_string(s, f),
        _ => todo!(),
    }
}

fn serialize_array(array: ArrayValue, f: &Formatter) -> Format {
    f.list([
        f.tag("Array@"),
        f.tag(array.range()),
        f.tag("("),
        f.indent(
            f.separated_list(
                array
                    .into_iter()
                    .map(|value| serialize_array_cell(value, f)),
                f.new_line(),
            ),
        ),
        f.new_line(),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_array_cell(cell: ArrayCell, f: &Formatter) -> Format {
    match cell {
        ArrayCell::Int(int_value) => serialize_int_value(int_value, f),
        _ => todo!(),
    }
}

fn serialize_int_value(cell: IntValue, f: &Formatter) -> Format {
    f.list([
        f.tag("Int@"),
        f.tag(cell.range()),
        f.tag("("),
        f.text(cell),
        f.tag(")"),
        f.tag(","),
    ])
}

fn serialize_string(s: StringValue, f: &Formatter) -> Format {
    f.list([
        f.tag("String@"),
        f.tag(s.range()),
        f.tag("("),
        f.text(s),
        f.tag(")"),
        f.tag(","),
    ])
}
