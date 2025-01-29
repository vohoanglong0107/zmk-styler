mod ast;
mod formatter;
mod parser;
use std::{error::Error, fs};

pub fn format(in_path: &str, out_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(in_path)?;
    let file_str = file.as_str();
    let tree = parser::parse(file_str)?;
    let formatted = formatter::format(tree);
    fs::write(out_path, formatted)?;
    Ok(())
}
