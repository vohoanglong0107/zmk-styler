use crate::{ast::AstNode, source::Source};

use super::ir::{self, Format};

pub(crate) struct Formatter<'src> {
    source: &'src Source<'src>,
}

impl<'src> Formatter<'src> {
    pub(crate) fn new(source: &'src Source<'src>) -> Self {
        Self { source }
    }
    /// Write node's content as it is.
    pub(crate) fn text<T: AstNode>(&self, node: T) -> Format {
        let node_text = std::str::from_utf8(&self.source[node.range()])
            .expect("Node must be a valid utf8 string");
        ir::text(node_text)
    }

    /// A normal text.
    pub(crate) fn tag(&self, text: impl ToString) -> Format {
        ir::text(text)
    }

    /// New line with indentation at current ident level
    pub(crate) fn new_line(&self) -> Format {
        ir::new_line()
    }

    pub(crate) fn space(&self) -> Format {
        self.tag(" ")
    }

    /// New line without indent
    pub(crate) fn empty_new_line(&self) -> Format {
        self.tag("\n")
    }

    /// Indents the specified block by one level on a new line
    pub(crate) fn indent(&self, doc: Format) -> Format {
        ir::indent(ir::concat([self.new_line(), doc]))
    }

    /// Concatenates a list of formatted text, separated by a specified separator
    pub(crate) fn separated_list(
        &self,
        documents: impl IntoIterator<Item = Format>,
        separator: Format,
    ) -> Format {
        ir::concat(itertools::intersperse(documents.into_iter(), separator))
    }

    /// Concatenates a list of formatted text
    pub(crate) fn list(&self, documents: impl IntoIterator<Item = Format>) -> Format {
        ir::concat(documents.into_iter())
    }

    /// Concatenates two formated text
    pub(crate) fn pair(&self, first: Format, second: Format) -> Format {
        self.list([first, second])
    }

    /// Do nothing
    pub(crate) fn nil(&self) -> Format {
        ir::nil()
    }
}

