use super::{
    config::Config,
    ir::{Concat, Format, Group, TextBreak, TextBreakKind},
};

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
            WriteContext::Concat => todo!(),
        }
    }

    fn write_concat(&mut self, Concat(formats): Concat) {
        formats
            .into_iter()
            .for_each(|format| self.write_with_context(format, WriteContext::Concat));
    }

    fn write_group(&mut self, Group(formats): Group) {
        let mut should_break = false;
        for format in formats.iter() {
            //FIXME: This is O(n^2) with n is the number of nested group
            should_break |= analyze(format);
        }
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

fn analyze(format: &Format) -> bool {
    match format {
        Format::TextBreak(text_break) => matches!(text_break.kind, TextBreakKind::NewLine),
        Format::Concat(subnodes) => {
            let mut should_break = false;
            for subformat in subnodes.0.iter() {
                should_break |= analyze(subformat)
            }
            should_break
        }
        Format::Group(subnodes) => {
            let mut should_break = false;
            for subformat in subnodes.0.iter() {
                should_break |= analyze(subformat)
            }
            should_break
        }
        _ => false,
    }
}
