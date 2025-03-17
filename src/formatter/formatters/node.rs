use crate::{
    ast::{AstNode, Identifier, Label, NodeBody, NodeBodyEntry, NodeDefinition},
    formatter::{
        ir::{text_break, TextBreakKind},
        rules::{
            format_leading_trivia, format_trailing_trivia, group, list, nil, pair, space, tag, text,
        },
        Format, FormatContext,
    },
};

use super::property::format_property;

pub(crate) fn format_node(node: &NodeDefinition, f: &mut FormatContext) -> Format {
    list([
        format_leading_trivia(f.trivia.leading_trivia(node.range()), f.source),
        format_label(node.label.as_ref(), f),
        format_identifier(&node.identifier, f),
        space(),
        format_node_body(&node.body, f),
        format_trailing_trivia(f.trivia.trailing_trivia(node.range()), f.source),
    ])
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
    list([
        group([
            tag("{"),
            text_break(0, TextBreakKind::Open),
            format_node_body_entries(&body.entries, f),
            format_leading_trivia(f.trivia.leading_trivia(body.r_curly.range), f.source),
            text_break(0, TextBreakKind::Close),
            tag("}"),
        ]),
        tag(";"),
    ])
}

fn format_node_body_entries(entries: &[NodeBodyEntry], f: &mut FormatContext) -> Format {
    let mut formatted = Vec::new();
    for entry in entries.iter() {
        let sep = text_break(0, TextBreakKind::NewLine);
        formatted.push(match entry {
            NodeBodyEntry::Node(node) => format_node(node, f),
            NodeBodyEntry::Property(property) => format_property(property, f),
        });
        formatted.push(sep);
    }
    list(formatted)
}
