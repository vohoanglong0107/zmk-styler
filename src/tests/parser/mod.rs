use std::fmt::Write;
mod node;
mod property;

use annotate_snippets::{Level, Renderer, Snippet};

use crate::{
    ast::{SyntaxNode, SyntaxNodeChild},
    formatter::{
        rules::{group, list, new_line, pair, tag, text_break},
        Format, FormatContext, TextBreakKind, Writer,
    },
    lexer::Token,
    parser::parse,
    source::Source,
};

fn debug_ast(test_str: &str) -> String {
    let source = Source::new(test_str);
    let (doc, token_source, diagnostics) = parse(&source);

    let formatter = FormatContext::new(&source, &token_source);
    let mut writer = Writer::default();
    let renderer = Renderer::plain();
    let mut diagnostic_message = String::new();
    for diagnostic in diagnostics {
        let range = diagnostic.range.limit(test_str.len());
        let message = Level::Error.title(&diagnostic.msg).snippet(
            Snippet::source(test_str)
                .line_start(token_source.get_line_number(range.start()) as usize)
                .fold(true)
                .annotation(Level::Error.span(range.into()).label(&diagnostic.msg)),
        );
        let message = renderer.render(message);
        writeln!(diagnostic_message, "{}\n", message).unwrap()
    }
    let syntax = writer.write(serialize_syntax(doc.syntax(), &formatter));
    format!("{diagnostic_message}{syntax}")
}

fn serialize_syntax(syntax: SyntaxNode, f: &FormatContext) -> Format {
    list([
        tag(format!("{:#?}@", syntax.kind)),
        tag(syntax.range),
        tag("("),
        group([
            text_break(0, TextBreakKind::Open),
            list(
                syntax
                    .children
                    .iter()
                    .cloned()
                    .map(|syntax| pair(serialize_syntax_child(syntax, f), new_line())),
            ),
            text_break(0, TextBreakKind::Close),
        ]),
        tag(")"),
    ])
}

fn serialize_syntax_child(syntax: SyntaxNodeChild, f: &FormatContext) -> Format {
    match syntax {
        SyntaxNodeChild::Token(token) => token_text(token, f.source),
        SyntaxNodeChild::Tree(syntax) => serialize_syntax(syntax, f),
    }
}

fn token_text(token: Token, source: &Source) -> Format {
    let token_text =
        std::str::from_utf8(&source[token.range]).expect("Node must be a valid utf8 string");
    tag(format!("{}@{}({})", token.kind, token.range, token_text))
}
