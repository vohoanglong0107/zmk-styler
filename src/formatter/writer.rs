use std::collections::LinkedList;

use super::{config::Config, ir::Format, rules::empty_new_line};

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
            Format::Concat(subnodes) => {
                let formalized = formalize_new_lines(subnodes.0);
                formalized.into_iter().for_each(|format| self.write(format));
            }

            Format::Nil => {}
        }
    }

    pub(crate) fn finish(&self) -> String {
        self.buffer.clone()
    }
}

fn formalize_new_lines(formats: LinkedList<Format>) -> Vec<Format> {
    let mut formalized = Vec::new();
    let mut iter = formats.into_iter();
    let mut format = iter.next();
    while format.is_some() {
        let mut continuous_trivia = Vec::new();
        while let Some(Format::Indent(indented)) = format {
            continuous_trivia.push(indented);
            format = iter.next();
        }
        // If user placed more than 2 consecutive new lines, preserve only 2 new lines
        if continuous_trivia
            .iter()
            .filter(|trivia| trivia.by_user)
            .count()
            > 1
        {
            let last = continuous_trivia.pop().unwrap();
            formalized.push(empty_new_line());
            formalized.push(Format::Indent(last));
        }
        // Else preserve only one newline
        else if let Some(indented) = continuous_trivia.pop() {
            formalized.push(Format::Indent(indented));
        }
        if let Some(format) = format {
            formalized.push(format);
        }
        format = iter.next();
    }
    formalized
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
