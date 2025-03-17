use std::collections::LinkedList;

use super::{
    config::Config,
    ir::{Format, TextBreakKind},
    rules::empty_new_line,
};

#[derive(Default)]
pub(crate) struct Writer {
    buffer: String,
    config: Config,
    current_indent_level: u32,
    num_bufferred_new_lines: u32,
    bufferred_discretionary_new_line: bool,
}

impl Writer {
    pub(crate) fn new(config: Config) -> Self {
        Writer {
            buffer: String::new(),
            config,
            current_indent_level: 0,
            num_bufferred_new_lines: 0,
            bufferred_discretionary_new_line: false,
        }
    }

    pub(crate) fn write(&mut self, node: Format) {
        match node {
            Format::Text(text) => {
                // Flush new line buffer
                for _ in 0..self.num_bufferred_new_lines {
                    self.buffer.push('\n');
                }
                if self.num_bufferred_new_lines > 0 {
                    for _ in 0..(self.current_indent_level * self.config.indent_width) {
                        self.buffer.push(' ')
                    }
                }
                self.bufferred_discretionary_new_line = false;
                self.num_bufferred_new_lines = 0;

                self.buffer.push_str(&text.0);
            }
            Format::TextBreak(text_break) => {}
            // Pre-order traversal
            Format::Concat(subnodes) => {
                let formalized = (subnodes.0);
                formalized.into_iter().for_each(|format| self.write(format));
            }
            Format::Group(subnodes) => {
                let mut should_break = false;
                for format in subnodes.0.iter() {
                    //FIXME: This is O(n^2)
                    should_break |= analyze(format);
                }
                let formalized = subnodes.0;
                if should_break {
                    for format in formalized {
                        if let Format::TextBreak(text_break) = format {
                            match text_break.kind {
                                TextBreakKind::Open => {
                                    self.current_indent_level += 1;
                                    if self.num_bufferred_new_lines == 0 {
                                        self.num_bufferred_new_lines += 1
                                    }
                                }
                                TextBreakKind::Close => {
                                    self.current_indent_level -= 1;
                                    if self.num_bufferred_new_lines == 0 {
                                        self.num_bufferred_new_lines += 1
                                    }
                                }
                                TextBreakKind::Discretion => {
                                    if self.num_bufferred_new_lines == 1
                                        && self.bufferred_discretionary_new_line
                                    {
                                        self.num_bufferred_new_lines += 1;
                                    } else if self.num_bufferred_new_lines == 1 {
                                        self.bufferred_discretionary_new_line = true
                                    } else if self.num_bufferred_new_lines == 0 {
                                        self.num_bufferred_new_lines += 1;
                                        self.bufferred_discretionary_new_line = true
                                    }
                                }
                                TextBreakKind::NewLine => {
                                    if self.num_bufferred_new_lines == 0 {
                                        self.num_bufferred_new_lines += 1
                                    }
                                }
                            }
                        } else {
                            self.write(format)
                        }
                    }
                } else {
                    for format in formalized {
                        if let Format::TextBreak(indented) = format {
                            for _ in 0..indented.size {
                                self.buffer.push(' ');
                            }
                        } else {
                            self.write(format)
                        }
                    }
                }
            }
            Format::Nil => {}
        }
    }

    pub(crate) fn finish(&self) -> String {
        self.buffer.clone()
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

#[cfg(test)]
mod test {
    use crate::formatter::ir::{concat, new_line, text};

    use super::Writer;
}
