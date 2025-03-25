mod config;
#[allow(clippy::module_inception)]
mod context;
mod formatters;
mod ir;
pub(crate) mod rules;
mod writer;

use crate::{ast::Document, source::Source, token_source::TokenSource};
pub(crate) use config::Config;
#[cfg(test)]
pub(crate) use formatters::format_document;
#[cfg(not(test))]
use formatters::format_document;

pub(crate) use context::FormatContext;
pub(crate) use ir::Format;
#[cfg(test)]
pub(crate) use ir::TextBreakKind;
pub(crate) use writer::Writer;

type FormatResult = Result<Format, ()>;

pub(crate) fn format(doc: Document, source: &Source, token_source: TokenSource) -> String {
    let mut format_context = FormatContext::new(source, &token_source);
    let format = format_document(doc, &mut format_context);
    let Ok(format) = format else {
        // FIXME: don't nuke user's file with syntax errors
        return "".to_owned();
    };

    let config = Config::default();
    let mut writer = Writer::new(config);
    writer.write(format)
}
