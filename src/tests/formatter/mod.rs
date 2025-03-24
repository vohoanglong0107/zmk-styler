use crate::{
    formatter::{format_document, Format, FormatContext, Writer},
    parser::parse,
    source::Source,
};

mod node;
mod property;
mod trivia;

fn debug_format(test_str: &str) -> Format {
    let source = Source::new(test_str);
    let (doc, comments) = parse(&source).unwrap();
    let mut format_context = FormatContext::new(&source, comments);
    format_document(doc, &mut format_context).unwrap()
}

fn debug_formatted(test_str: &str) -> String {
    let document = debug_format(test_str);
    let mut writer = Writer::default();
    writer.write(document)
}
