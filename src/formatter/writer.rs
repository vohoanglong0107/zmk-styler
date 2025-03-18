use super::{
    config::Config,
    ir::{Concat, Format, Group, TextBreak, TextBreakKind},
};

#[cfg(test)]
use std::cell::Cell;

#[derive(Default)]
pub(crate) struct Writer {
    buffer: String,
    new_line_tracker: NewLineTracker,
}

impl Writer {
    pub(crate) fn new(config: Config) -> Self {
        Writer {
            buffer: String::new(),
            new_line_tracker: NewLineTracker::new(config),
        }
    }

    pub(crate) fn write(&mut self, node: Format) -> String {
        self.write_with_context(node, WriteContext::Concat);
        self.buffer.clone()
    }

    fn write_with_context(&mut self, node: Format, context: WriteContext) {
        match node {
            Format::Text(text) => self.write_text(&text.0),
            Format::TextBreak(text_break) => self.write_text_break(text_break, context),
            Format::Concat(concat) => self.write_concat(concat),
            Format::Group(group) => self.write_group(group),
            Format::Nil => {}
        }
    }

    fn write_text(&mut self, text: &str) {
        self.buffer.push_str(&self.new_line_tracker.flush());
        self.buffer.push_str(text);
    }

    fn write_text_break(&mut self, text_break: TextBreak, context: WriteContext) {
        match context {
            WriteContext::Group { break_to_lines } => {
                if break_to_lines {
                    match text_break.kind {
                        TextBreakKind::Open => self.new_line_tracker.indent(),
                        TextBreakKind::Close => self.new_line_tracker.dedent(),
                        TextBreakKind::Discretion => self.new_line_tracker.buffer_discretion(),
                        TextBreakKind::NewLine => self.new_line_tracker.buffer_new_line(),
                        TextBreakKind::Same => self.new_line_tracker.buffer_new_line(),
                    }
                } else {
                    for _ in 0..text_break.size {
                        self.buffer.push(' ');
                    }
                }
            }
            WriteContext::Concat => self.new_line_tracker.buffer_new_line(),
        }
    }

    fn write_concat(&mut self, Concat(formats): Concat) {
        formats
            .into_iter()
            .for_each(|format| self.write_with_context(format, WriteContext::Concat));
    }

    fn write_group(
        &mut self,
        Group {
            mut formats,
            broken_to_multilines,
        }: Group,
    ) {
        let should_break = match broken_to_multilines {
            Some(should_break) => should_break,
            None => formats.iter_mut().any(analyze),
        };

        for format in formats {
            self.write_with_context(
                format,
                WriteContext::Group {
                    break_to_lines: should_break,
                },
            )
        }
    }
}

enum WriteContext {
    Concat,
    Group { break_to_lines: bool },
}

struct NewLineTracker {
    config: Config,
    current_indent_level: u32,
    num_bufferred_new_lines: u32,
    bufferred_discretionary_new_line: bool,
}

impl NewLineTracker {
    fn new(config: Config) -> Self {
        Self {
            config,
            current_indent_level: 0,
            num_bufferred_new_lines: 0,
            bufferred_discretionary_new_line: false,
        }
    }

    fn flush(&mut self) -> String {
        let mut output = String::new();
        for _ in 0..self.num_bufferred_new_lines {
            output.push('\n');
        }
        if self.num_bufferred_new_lines > 0 {
            for _ in 0..(self.current_indent_level * self.config.indent_width) {
                output.push(' ')
            }
        }
        self.bufferred_discretionary_new_line = false;
        self.num_bufferred_new_lines = 0;
        output
    }

    fn indent(&mut self) {
        self.current_indent_level += 1;
        self.buffer_new_line();
    }

    fn dedent(&mut self) {
        self.current_indent_level -= 1;
        self.buffer_new_line();
    }

    fn buffer_new_line(&mut self) {
        if self.num_bufferred_new_lines == 0 {
            self.num_bufferred_new_lines += 1
        }
    }

    fn buffer_discretion(&mut self) {
        if self.num_bufferred_new_lines == 1 && self.bufferred_discretionary_new_line {
            self.num_bufferred_new_lines += 1;
        } else {
            self.buffer_new_line();
        }
        self.bufferred_discretionary_new_line = true
    }
}

impl Default for NewLineTracker {
    fn default() -> Self {
        NewLineTracker::new(Config::default())
    }
}

#[cfg(test)]
thread_local! {
    static COUNTER: Cell<usize> = const { Cell::new(0) };
}

fn analyze(format: &mut Format) -> bool {
    #[cfg(test)]
    COUNTER.with(|c| c.set(c.get() + 1));

    match format {
        Format::TextBreak(text_break) => matches!(text_break.kind, TextBreakKind::NewLine),
        Format::Concat(subnodes) => subnodes.0.iter_mut().any(analyze),
        Format::Group(group) => {
            let should_break = match group.broken_to_multilines {
                Some(should_break) => should_break,
                None => group.formats.iter_mut().any(analyze),
            };
            group.broken_to_multilines = Some(should_break);
            should_break
        }
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::formatter::{
        rules::{group, tag},
        writer::COUNTER,
        Writer,
    };

    #[test]
    fn test_analyze_performance() {
        let mut test_format = tag("testing");
        for _ in 0..1000 {
            test_format = group([test_format])
        }
        let mut writer = Writer::default();
        COUNTER.with(|c| c.set(0));
        writer.write(test_format);
        // Don't need to be exact, just shouldn't be millions
        COUNTER.with(|c| assert_eq!(c.get(), 1000));
    }
}
