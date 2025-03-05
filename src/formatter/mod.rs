mod config;
#[allow(clippy::module_inception)]
mod formatter;
mod formatters;
mod ir;
mod writer;

use crate::{ast::Document, source::Source};
pub(crate) use config::Config;
#[cfg(test)]
pub(crate) use formatters::format_document;
#[cfg(not(test))]
use formatters::format_document;

pub(crate) use formatter::Formatter;
pub(crate) use ir::Format;
pub(crate) use writer::Writer;

pub(crate) fn format(doc: Document, source: &Source) -> String {
    let formatter = Formatter::new(source);
    let format = format_document(doc, &formatter);

    let config = Config::default();
    let mut writer = Writer::new(config);
    writer.write(format);
    writer.finish()
}
