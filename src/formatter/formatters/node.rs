use crate::{
    ast::{Identifier, Label, NodeBody, NodeBodyEntry, NodeDefinition, PropertyDefinition},
    formatter::{Format, Formatter},
};

use super::property::format_property;

pub(crate) fn format_node(node: NodeDefinition, f: &Formatter) -> Format {
    f.list([
        format_label(node.label, f),
        format_identifier(node.identifier, f),
        format_node_body(node.body, f),
    ])
}

fn format_label(label: Option<Label>, f: &Formatter) -> Format {
    match label {
        // TODO:: format label text and ":" separately to prevent comments
        // and whitespaces in between
        Some(label) => f.pair(f.text(label), f.space()),
        None => f.nil(),
    }
}

fn format_identifier(identifier: Identifier, f: &Formatter) -> Format {
    match identifier {
        Identifier::Root(_) => f.tag("/"),
        Identifier::Other(identifier) => match identifier.address {
            Some(address) => f.list([f.text(identifier.name), f.tag("@"), f.text(address)]),
            None => f.text(identifier.name),
        },
    }
}

fn format_node_body(body: NodeBody, f: &Formatter) -> Format {
    let multiline = !body.entries.is_empty();

    f.list([
        f.space(),
        f.tag("{"),
        if multiline {
            f.indent(format_node_body_entries(body.entries, f))
        } else {
            f.nil()
        },
        if multiline { f.new_line() } else { f.nil() },
        f.tag("}"),
        f.tag(";"),
    ])
}

fn format_node_body_entries(entries: Vec<NodeBodyEntry>, f: &Formatter) -> Format {
    f.separated_list(
        entries.into_iter().map(|entry| match entry {
            NodeBodyEntry::Node(node) => format_node(node, f),
            NodeBodyEntry::Property(property) => format_property(property, f),
        }),
        f.new_line(),
    )
}
