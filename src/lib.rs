mod ast;
mod formatter;
mod lexer;
mod parser;
mod source;
mod trivia;
use std::{error::Error, fs};

use source::Source;

#[cfg(test)]
mod tests;

pub fn format(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(in_path)?;
    let file_str = file.as_str();

    let source = Source::new(file_str);
    let (doc, comments) = parser::parse(&source)?;
    let formatted = formatter::format(doc, &source, comments);
    fs::write(out_path, formatted)?;
    Ok(())
}
