use std::{collections::LinkedList, fmt::Debug};

/// Text Verbatim
#[derive(Clone)]
pub(crate) struct Text(pub String);

/// Indented block of text
#[derive(Clone)]
pub(crate) struct Indent {
    pub(super) level: u32,
    pub(super) by_user: bool,
}

/// Concatination of Format nodes
#[derive(Clone)]
pub(crate) struct Concat(pub LinkedList<Format>);

#[derive(Clone)]
pub(crate) enum Format {
    Text(Text),
    Indent(Box<Indent>),
    Concat(Box<Concat>),
    Nil,
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text({})", self.0)
    }
}

impl Debug for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Indent({})", self.level)
    }
}

impl Debug for Concat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Concat ")?;
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(arg0) => write!(f, "{:#?}", arg0),
            Self::Indent(arg0) => write!(f, "{:#?}", arg0),
            Self::Concat(arg0) => write!(f, "{:#?}", arg0),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

pub(super) fn text(text: impl ToString) -> Format {
    Format::Text(Text(text.to_string()))
}

pub(super) fn new_line() -> Format {
    Format::Indent(Box::new(Indent {
        level: 0,
        by_user: false,
    }))
}

pub(super) fn user_placed_new_line() -> Format {
    Format::Indent(Box::new(Indent {
        level: 0,
        by_user: true,
    }))
}

/// Increases the indent level of the specified block
pub(super) fn indent(document: Format) -> Format {
    match document {
        // nest i (text s) = text s
        Format::Text(text) => Format::Text(text),
        // nest i (nest j x) = nest (i + j) x
        Format::Indent(indented) => Format::Indent(Box::new(Indent {
            level: indented.level + 1,
            by_user: indented.by_user,
        })),
        // nest i (x <> y) = nest i x <> nest i y
        Format::Concat(formats) => {
            let formats = formats.0.into_iter().map(indent).collect();
            Format::Concat(Box::new(Concat(formats)))
        }
        // nest i nil = nil
        Format::Nil => Format::Nil,
    }
}

/// Concatenates multi sub formats
pub(super) fn concat(formats: impl IntoIterator<Item = Format>) -> Format {
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
    Format::Concat(Box::new(Concat(combined)))
}

pub(crate) fn nil() -> Format {
    Format::Nil
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    /// nest i (text s) = text s
    #[test]
    fn nest_text_eq_text() {
        let format = indent(indent(indent(text("abc".to_string()))));
        let Format::Text(fomat) = format else {
            panic!("This test format must be a text {:#?}", format);
        };
        assert_eq!(fomat.0, "abc")
    }

    /// i ‘Line‘ x = nest i line <> x
    #[test]
    fn nest_line_eq_nest() {
        let format = indent(new_line());
        let Format::Indent(format) = format else {
            panic!("This test format must be an indent {:#?}", format);
        };
        assert_eq!(format.level, 1)
    }

    /// nest i (nest j x) = nest (i + j) x
    #[test]
    fn nest_line_eq_bigger_nest() {
        let format = indent(indent(indent(new_line())));
        let Format::Indent(format) = format else {
            panic!("This test format must be an indent {:#?}", format);
        };
        assert_eq!(format.level, 3)
    }
}
