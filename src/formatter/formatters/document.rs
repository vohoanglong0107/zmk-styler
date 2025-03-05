use crate::{
    ast::{Document, Statement},
    formatter::{Format, Formatter},
};

use super::node::format_node;

pub(crate) fn format_document(document: Document, f: &Formatter) -> Format {
    f.list(
        document
            .statements
            .into_iter()
            .map(|statement| format_statement(statement, f)),
    )
}

fn format_statement(statement: Statement, f: &Formatter) -> Format {
    match statement {
        Statement::Node(node) => format_node(node, f),
    }
}
