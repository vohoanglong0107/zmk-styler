use config::Config;
use formatters::format_node;
use writer::Writer;

use crate::ast::Node;

mod config;
mod formatters;
mod ir;
mod writer;

pub(crate) fn format(node: Node) -> String {
    let config = Config::default();
    let mut writer = Writer::new(config);
    let document = format_node(node);
    writer.write(document);
    writer.output()
}
