mod ast;
mod formatter;
mod parser;
use std::{error::Error, fs};

use formatter::format;
use parser::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "glove80.keymap";
    let file = fs::read_to_string(filename)?;
    let file_str = file.as_str();
    let tree = parse(file_str)?;
    println!("Parsed tree: {tree:#?}");
    let formatted = format(tree);
    fs::write("formatted.keymap", formatted)?;
    Ok(())
}
