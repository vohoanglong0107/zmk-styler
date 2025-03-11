use crate::{
    ast::{Identifier, Label, NodeBody, NodeBodyEntry, NodeDefinition},
    formatter::{
        rules::{
            flush_comments, indent, list, new_line, nil, pair, separated_list, space, tag, text,
        },
        Format, FormatContext,
    },
};

use super::property::format_property;

pub(crate) fn format_node(node: NodeDefinition, f: &mut FormatContext) -> Format {
    let node_format = [
        flush_comments(&node, f.source, &mut f.trivia),
        format_label(node.label, f),
        format_identifier(node.identifier, f),
        format_node_body(node.body, f),
    ];
    list(node_format)
}

fn format_label(label: Option<Label>, f: &FormatContext) -> Format {
    match label {
        // TODO:: format label text and ":" separately to prevent comments
        // and whitespaces in between
        Some(label) => pair(text(label, f.source), space()),
        None => nil(),
    }
}

fn format_identifier(identifier: Identifier, f: &FormatContext) -> Format {
    match identifier {
        Identifier::Root(_) => tag("/"),
        Identifier::Other(identifier) => match identifier.address {
            Some(address) => list([
                text(identifier.name, f.source),
                tag("@"),
                text(address, f.source),
            ]),
            None => text(identifier.name, f.source),
        },
    }
}

fn format_node_body(body: NodeBody, f: &mut FormatContext) -> Format {
    let multiline = !body.entries.is_empty();

    list([
        space(),
        tag("{"),
        if multiline {
            indent(format_node_body_entries(body.entries, f))
        } else {
            nil()
        },
        if multiline { new_line() } else { nil() },
        tag("}"),
        tag(";"),
    ])
}

fn format_node_body_entries(entries: Vec<NodeBodyEntry>, f: &mut FormatContext) -> Format {
    separated_list(
        entries.into_iter().map(|entry| match entry {
            NodeBodyEntry::Node(node) => format_node(node, f),
            NodeBodyEntry::Property(property) => format_property(property, f),
        }),
        new_line(),
    )
}
