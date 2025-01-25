use super::{config::Config, ir::Document};

pub(super) struct Writer {
    buffer: String,
    config: Config,
}

impl Writer {
    pub(super) fn new(config: Config) -> Self {
        Writer {
            buffer: String::new(),
            config,
        }
    }
    pub(super) fn write(&mut self, node: Document) {
        match node {
            Document::Text(text) => self.buffer.push_str(&text.0),
            Document::Indent(indent) => {
                self.buffer.push('\n');
                for _ in 0..(indent.level * self.config.indent_width) {
                    self.buffer.push(' ')
                }
            }
            // Pre-order traversal
            Document::Concat(subnodes) => subnodes.0.into_iter().for_each(|doc| self.write(doc)),

            Document::Nil => {}
        }
    }

    pub(super) fn output(&self) -> String {
        self.buffer.clone()
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
        let mut writer = new_writer();
        let doc = indent(indent(indent(empty_new_line())));
        writer.write(doc);
        assert_eq!(writer.output(), "\n")
    }

    #[test]
    fn indented_newline() {
        let mut writer = new_writer();
        let doc = concat(vec![
            text("abc"),
            indent(concat(vec![new_line(), text("xyz")])),
        ]);
        writer.write(doc);
        assert_eq!(
            writer.output(),
            r#"abc
    xyz"#
        )
    }

    #[test]
    fn nested_concat() {
        let mut writer = new_writer();
        let doc = concat(vec![
            concat(vec![text("abc"), text("def")]),
            indent(concat(vec![new_line(), text("ghi")])),
            concat(vec![
                new_line(),
                text("jkl"),
                concat(vec![text("mno"), text("prq")]),
            ]),
        ]);
        writer.write(doc);
        assert_eq!(
            writer.output(),
            r#"abcdef
    ghi
jklmnoprq"#
        )
    }

    fn new_writer() -> Writer {
        Writer::new(Config::default())
    }
}
