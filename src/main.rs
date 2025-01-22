mod ast;
mod parser;
use std::{error::Error, fs};

use parser::parse;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "glove80.keymap";
    let file = fs::read_to_string(filename)?;
    let file_str = file.as_str();
    let tree = parse(file_str)?;
    println!("Parsed tree: {tree:#?}");
    Ok(())
}
