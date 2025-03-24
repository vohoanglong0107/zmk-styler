use crate::{
    ast::{
        AstNode, Label, NodeBody, NodeBodyEntries, NodeBodyEntry, NodeDefinition, NodeIdentifier,
    },
    formatter::{
        ir::{text_break, TextBreakKind},
        rules::{
            format_leading_trivia, format_trailing_trivia, group, list, nil, pair, space, tag, text,
        },
        Format, FormatContext, FormatResult,
    },
};

use super::property::format_property;

pub(crate) fn format_node(node: NodeDefinition, f: &mut FormatContext) -> FormatResult {
    let label = node.label();
    let identifier = node.identifier()?;
    let body = node.body()?;
    Ok(list([
        format_leading_trivia(f.trivia.leading_trivia(node.range()), f.source),
        label.map_or(nil(), |label| format_label(label, f)),
        format_identifier(identifier, f)?,
        space(),
        format_node_body(body, f)?,
        format_trailing_trivia(f.trivia.trailing_trivia(node.range()), f.source),
    ]))
}

fn format_label(label: Label, f: &FormatContext) -> Format {
    // TODO:: format label text and ":" separately to prevent comments
    // and whitespaces in between
    pair(text(&label, f.source), space())
}

fn format_identifier(identifier: NodeIdentifier, f: &FormatContext) -> FormatResult {
    let format = match identifier {
        NodeIdentifier::Root(_) => tag("/"),
        NodeIdentifier::NonRoot(identifier) => match identifier.address() {
            Some(address) => list([
                text(&identifier.name()?, f.source),
                tag("@"),
                text(&address, f.source),
            ]),
            None => text(&identifier.name()?, f.source),
        },
    };
    Ok(format)
}

fn format_node_body(body: NodeBody, f: &mut FormatContext) -> FormatResult {
    Ok(list([
        group([
            tag("{"),
            text_break(0, TextBreakKind::Open),
            format_node_body_entries(body.entries()?, f)?,
            format_leading_trivia(f.trivia.leading_trivia(body.r_curly()?.range), f.source),
            text_break(0, TextBreakKind::Close),
            tag("}"),
        ]),
        tag(";"),
    ]))
}

fn format_node_body_entries(entries: NodeBodyEntries, f: &mut FormatContext) -> FormatResult {
    let mut formatted = Vec::new();
    for entry in entries.into_iter() {
        let sep = text_break(0, TextBreakKind::NewLine);
        formatted.push(match entry {
            NodeBodyEntry::Node(node) => format_node(node, f)?,
            NodeBodyEntry::Property(property) => format_property(property, f)?,
        });
        formatted.push(sep);
    }
    Ok(list(formatted))
}
