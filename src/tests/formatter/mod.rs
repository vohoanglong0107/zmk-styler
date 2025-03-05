use crate::{
    formatter::{format_document, Format, Formatter, Writer},
    parser::parse,
    source::Source,
};

mod node;
mod property;

fn debug_format(test_str: &str) -> Format {
    let source = Source::new(test_str);
    let formatter = Formatter::new(&source);
    let doc = parse(&source).unwrap();
    format_document(doc, &formatter)
}

fn debug_formatted(test_str: &str) -> String {
    let source = Source::new(test_str);
    let formatter = Formatter::new(&source);
    let doc = parse(&source).unwrap();
    let document = format_document(doc, &formatter);

    let mut writer = Writer::default();
    writer.write(document);
    writer.finish()
}
