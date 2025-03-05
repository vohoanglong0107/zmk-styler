use super::{config::Config, ir::Format};

#[derive(Default)]
pub(crate) struct Writer {
    buffer: String,
    config: Config,
}

impl Writer {
    pub(crate) fn new(config: Config) -> Self {
        Writer {
            buffer: String::new(),
            config,
        }
    }
    pub(crate) fn write(&mut self, node: Format) {
        match node {
            Format::Text(text) => self.buffer.push_str(&text.0),
            Format::Indent(indent) => {
                self.buffer.push('\n');
                for _ in 0..(indent.level * self.config.indent_width) {
                    self.buffer.push(' ')
                }
            }
            // Pre-order traversal
            Format::Concat(subnodes) => subnodes.0.into_iter().for_each(|doc| self.write(doc)),

            Format::Nil => {}
        }
    }

    pub(crate) fn finish(&self) -> String {
        self.buffer.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::formatter::ir::{concat, indent, new_line, text};

    use super::Writer;

    #[test]
    fn empty_new_line_does_not_indent() {
        let mut writer = Writer::default();
        let doc = indent(indent(indent(text("\n"))));
        writer.write(doc);
        assert_eq!(writer.finish(), "\n")
    }

    #[test]
    fn indented_newline() {
        let mut writer = Writer::default();
        let doc = concat(vec![
            text("abc"),
            indent(concat(vec![new_line(), text("xyz")])),
        ]);
        writer.write(doc);
        assert_eq!(
            writer.finish(),
            r#"abc
    xyz"#
        )
    }

    #[test]
    fn nested_concat() {
        let mut writer = Writer::default();
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
            writer.finish(),
            r#"abcdef
    ghi
jklmnoprq"#
        )
    }
}
