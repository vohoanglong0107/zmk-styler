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
            Document::Nil => "".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::formatter::{
        config::Config,
        ir::{concat, empty_new_line, indent, new_line, text},
    };

    use super::Writer;

    #[test]
    fn empty_new_line_does_not_indent() {
        let writer = new_writer();
        let doc = indent(indent(indent(empty_new_line())));
        assert_eq!(writer.write(doc), "\n")
    }

    #[test]
    fn indented_newline() {
        let writer = new_writer();
        let doc = concat(vec![
            text("abc"),
            indent(concat(vec![new_line(), text("xyz")])),
        ]);
        assert_eq!(
            writer.write(doc),
            r#"abc
    xyz"#
        )
    }

    #[test]
    fn nested_concat() {
        let writer = new_writer();
        let doc = concat(vec![
            concat(vec![text("abc"), text("def")]),
            indent(concat(vec![new_line(), text("ghi")])),
            concat(vec![
                new_line(),
                text("jkl"),
                concat(vec![text("mno"), text("prq")]),
            ]),
        ]);
        assert_eq!(
            writer.write(doc),
            r#"abcdef
    ghi
jklmnoprq"#
        )
    }

    fn new_writer() -> Writer {
        Writer {
            config: Config::default(),
        }
    }
}
