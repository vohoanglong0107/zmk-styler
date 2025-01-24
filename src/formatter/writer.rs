use super::{config::Config, ir::Document};

pub(super) struct Writer {
    config: Config,
}

impl Writer {
    pub(super) fn new(config: Config) -> Self {
        Writer { config }
    }
    pub(super) fn write(&self, node: Document) -> String {
        match node {
            Document::Text(text) => text.0,
            Document::Indent(indent) => {
                // Can't panic. Who on earth needs that much indent?
                let indent_spaces: usize = (indent.level * self.config.indent_width)
                    .try_into()
                    .unwrap();
                format!("\n{}", " ".repeat(indent_spaces))
            }
            Document::Concat(subnodes) => {
                let texts = subnodes
                    .0
                    .into_iter()
                    .map(|subnode| self.write(subnode))
                    .collect::<Vec<_>>();
                texts.join("")
            }
        }
    }
}
