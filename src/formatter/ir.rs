use std::fmt::Debug;

/// Text Verbatim
#[derive(Debug, Clone)]
pub(super) struct Text(pub String);
/// Indented block of text
#[derive(Debug, Clone)]
pub(super) struct Indent {
    pub(super) level: u32,
}
/// Concatination of Document nodes
#[derive(Debug, Clone)]
pub(super) struct Concat(pub Vec<Document>);

#[derive(Clone)]
pub(super) enum Document {
    Text(Text),
    Indent(Box<Indent>),
    Concat(Box<Concat>),
    Nil,
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(arg0) => write!(f, "{:#?}", arg0),
            Self::Indent(arg0) => write!(f, "{:#?}", arg0),
            Self::Concat(arg0) => write!(f, "{:#?}", arg0),
            Self::Nil => write!(f, "Nil"),
        }
    }
}

pub(super) fn text(text: impl ToString) -> Document {
    Document::Text(Text(text.to_string()))
}

pub(super) fn tag(tag: &str) -> Document {
    Document::Text(Text(tag.to_string()))
}

pub(super) fn space() -> Document {
    tag(" ")
}

/// New line without indent
pub(super) fn empty_new_line() -> Document {
    tag("\n")
}

/// New line with indent
pub(super) fn new_line() -> Document {
    Document::Indent(Box::new(Indent { level: 0 }))
}

/// Indent a document
pub(super) fn indent(document: Document) -> Document {
    match document {
        // nest i (text s) = text s
        Document::Text(text) => Document::Text(text),
        // nest i (nest j x) = nest (i + j) x
        Document::Indent(indented) => Document::Indent(Box::new(Indent {
            level: indented.level + 1,
        })),
        // nest i (x <> y) = nest i x <> nest i y
        Document::Concat(docs) => {
            let docs = docs.0.into_iter().map(indent).collect();
            Document::Concat(Box::new(Concat(docs)))
        }
        // nest i nil = nil
        Document::Nil => Document::Nil,
    }
}

/// Concatenates multi document
pub(super) fn concat(documents: impl IntoIterator<Item = Document>) -> Document {
    Document::Concat(Box::new(Concat(
        documents
            .into_iter()
            .filter(|doc| !matches!(doc, Document::Nil))
            .collect(),
    )))
}

pub(super) fn nil() -> Document {
    Document::Nil
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    /// nest i (text s) = text s
    #[test]
    fn nest_text_eq_text() {
        let doc = indent(indent(indent(text("abc".to_string()))));
        let Document::Text(doc) = doc else {
            panic!("This test doc must be a text {:#?}", doc);
        };
        assert_eq!(doc.0, "abc")
    }

    /// i ‘Line‘ x = nest i line <> x
    #[test]
    fn nest_line_eq_nest() {
        let doc = indent(new_line());
        let Document::Indent(doc) = doc else {
            panic!("This test doc must be an indent {:#?}", doc);
        };
        assert_eq!(doc.level, 1)
    }

    /// nest i (nest j x) = nest (i + j) x
    #[test]
    fn nest_line_eq_bigger_nest() {
        let doc = indent(indent(indent(new_line())));
        let Document::Indent(doc) = doc else {
            panic!("This test doc must be an indent {:#?}", doc);
        };
        assert_eq!(doc.level, 3)
    }
}
