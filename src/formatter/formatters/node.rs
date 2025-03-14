use crate::{
    ast::{AstNode, Identifier, Label, NodeBody, NodeBodyEntry, NodeDefinition},
    formatter::{
        rules::{
            format_leading_trivia, format_trailing_trivia, indent, list, new_line, nil, pair,
            separated_list, space, tag, text,
        },
        Format, FormatContext,
    },
};

use super::property::format_property;

pub(crate) fn format_node(node: &NodeDefinition, f: &mut FormatContext) -> Format {
    let node_format = [
        format_leading_trivia(node.range(), f.source, &mut f.trivia),
        format_label(node.label.as_ref(), f),
        format_identifier(&node.identifier, f),
        format_node_body(&node.body, f),
        format_trailing_trivia(node.range(), f.source, &mut f.trivia),
    ];
    list(node_format)
}

fn format_label(label: Option<&Label>, f: &FormatContext) -> Format {
    match label {
        // TODO:: format label text and ":" separately to prevent comments
        // and whitespaces in between
        Some(label) => pair(text(label, f.source), space()),
        None => nil(),
    }
}

fn format_identifier(identifier: &Identifier, f: &FormatContext) -> Format {
    match identifier {
        Identifier::Root(_) => tag("/"),
        Identifier::Other(identifier) => match &identifier.address {
            Some(address) => list([
                text(&identifier.name, f.source),
                tag("@"),
                text(address, f.source),
            ]),
            None => text(&identifier.name, f.source),
        },
    }
}

fn format_node_body(body: &NodeBody, f: &mut FormatContext) -> Format {
    let multiline = !body.entries.is_empty();

    list([
        space(),
        tag("{"),
        if multiline {
            indent(pair(
                format_node_body_entries(&body.entries, f),
                format_leading_trivia(body.r_curly.range, f.source, &mut f.trivia),
            ))
        } else {
            nil()
        },
        if multiline { new_line() } else { nil() },
        tag("}"),
        tag(";"),
    ])
}

fn format_node_body_entries(entries: &[NodeBodyEntry], f: &mut FormatContext) -> Format {
    let mut formatted = Vec::new();
    let num_entries = entries.len();
    for (pos, entry) in entries.iter().enumerate() {
        let sep = if pos != num_entries - 1 {
            new_line()
        } else {
            nil()
        };
        formatted.push(match entry {
            NodeBodyEntry::Node(node) => format_node(node, f),
            NodeBodyEntry::Property(property) => format_property(property, f),
        });
        formatted.push(sep);
    }
    list(formatted)
}
