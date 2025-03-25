mod ast;
mod formatter;
mod lexer;
mod parser;
mod source;
mod token_source;
use std::{error::Error, fs};

use source::Source;

#[cfg(test)]
mod tests;

pub fn format(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(in_path)?;
    let file_str = file.as_str();

    let source = Source::new(file_str);
    let (doc, token_source, _) = parser::parse(&source);
    let formatted = formatter::format(doc, &source, token_source);
    fs::write(out_path, formatted)?;
    Ok(())
}
