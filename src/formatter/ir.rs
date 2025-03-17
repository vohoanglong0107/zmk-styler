use std::{collections::LinkedList, fmt::Debug};

/// Text Verbatim
#[derive(Clone)]
pub(crate) struct Text(pub String);

/// Break text into new line
/// Or expand to white space of `size` size
#[derive(Clone)]
pub(crate) struct TextBreak {
    pub(super) size: u32,
    pub(super) kind: TextBreakKind,
}

#[derive(Clone, Debug)]
pub(crate) enum TextBreakKind {
    /// Increase the indent level by one
    Open,
    /// Keep the indentation level
    Same,
    /// Lower the indent level by one
    Close,
    /// Must break to a new line, while keeping the indentation level
    NewLine,
    /// Respect user's line breaks
    Discretion,
}

/// Concatenation of Format nodes
#[derive(Clone)]
pub(crate) struct Concat(pub LinkedList<Format>);

/// Group all direct children on a single line,
/// or break all of them to multiple lines
#[derive(Clone)]
pub(crate) struct Group(pub LinkedList<Format>);

#[derive(Clone)]
pub(crate) enum Format {
    Text(Text),
    TextBreak(TextBreak),
    Concat(Concat),
    Group(Group),
    Nil,
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text({})", self.0)
    }
}

impl Debug for TextBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextBreak({},{:?})", self.size, self.kind)
    }
}

impl Debug for Concat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Concat ")?;
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group ")?;
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(arg0) => write!(f, "{:#?}", arg0),
            Self::TextBreak(arg0) => write!(f, "{:#?}", arg0),
            Self::Concat(arg0) => write!(f, "{:#?}", arg0),
            Self::Group(arg0) => write!(f, "{:#?}", arg0),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

pub(super) fn text(text: impl ToString) -> Format {
    Format::Text(Text(text.to_string()))
}

pub(super) fn text_break(size: u32, kind: TextBreakKind) -> Format {
    Format::TextBreak(TextBreak { size, kind })
}

pub(super) fn new_line() -> Format {
    Format::TextBreak(TextBreak {
        size: 0,
        kind: TextBreakKind::NewLine,
    })
}

/// Concatenates multi sub formats
pub(super) fn concat(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Concat(Concat(expand_concatenated_concat(formats)))
}

/// Group all formatted text on a single line
/// Or break them to multiple lines
pub(super) fn group(formats: impl IntoIterator<Item = Format>) -> Format {
    Format::Group(Group(expand_concatenated_concat(formats)))
}

fn expand_concatenated_concat(formats: impl IntoIterator<Item = Format>) -> LinkedList<Format> {
    let mut combined = LinkedList::new();
    for format in formats {
        match format {
            Format::Concat(mut sub_formats) => {
                combined.append(&mut sub_formats.0);
            }
            Format::Nil => {}
            _ => combined.push_back(format),
        }
    }
    combined
}

pub(crate) fn nil() -> Format {
    Format::Nil
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
}
