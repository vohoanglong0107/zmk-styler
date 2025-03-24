use crate::{
    ast::{Document, Statement},
    formatter::{rules::list, Format, FormatContext, FormatResult},
};

use super::node::format_node;

pub(crate) fn format_document(document: Document, f: &mut FormatContext) -> FormatResult {
    Ok(list(
        document
            .statements()
            .into_iter()
            .map(|statement| format_statement(statement, f))
            .collect::<Result<Vec<Format>, ()>>()?,
    ))
}

fn format_statement(statement: Statement, f: &mut FormatContext) -> FormatResult {
    match statement {
        Statement::Node(node) => format_node(node, f),
    }
}
