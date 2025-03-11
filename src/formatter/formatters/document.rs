use crate::{
    ast::{Document, Statement},
    formatter::{rules::list, Format, FormatContext},
};

use super::node::format_node;

pub(crate) fn format_document(document: Document, f: &mut FormatContext) -> Format {
    list(
        document
            .statements
            .into_iter()
            .map(|statement| format_statement(statement, f)),
    )
}

fn format_statement(statement: Statement, f: &mut FormatContext) -> Format {
    match statement {
        Statement::Node(node) => format_node(node, f),
    }
}
