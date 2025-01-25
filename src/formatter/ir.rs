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

pub(super) fn text(text: String) -> Document {
    Document::Text(Text(text))
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
/// FIXME: this is O(n ^ 2) with n is the level of nested concat
/// For example, concat(concat(concat(concat(...)))) runtime is O(n ^ 2)
pub(super) fn concat(documents: Vec<Document>) -> Document {
    let mut expanded_documents = Vec::new();
    for doc in documents {
        match doc {
            // x <> (y <> z) = x <> y <> z
            Document::Concat(nested_docs) => expanded_documents.extend((*nested_docs).0),
            Document::Text(_) | Document::Indent(_) => expanded_documents.push(doc),
            // x <> nil = x
            Document::Nil => {}
        }
    }
    Document::Concat(Box::new(Concat(expanded_documents)))
}

pub(super) fn nil() -> Document {
    Document::Nil
}

#[cfg(test)]
mod test {
    use core::panic;

    use super::*;
    #[test]
    fn ensure_flatten_document() {
        let doc = concat(vec![
            text("abc".to_string()),
            new_line(),
            concat(vec![new_line(), text("def".to_string())]),
            new_line(),
            text("xyz".to_string()),
        ]);
        let Document::Concat(doc) = doc else {
            panic!("This test doc must be a list texts");
        };
        let doc = (*doc).0;
        for nested_doc in doc {
            if matches!(nested_doc, Document::Concat(_)) {
                panic!("There must be no nested concat")
            }
        }
    }
}
