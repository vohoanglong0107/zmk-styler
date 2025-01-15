mod node;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "glove80.keymap";
    let file = fs::read_to_string(filename)?;
    for char in file.chars() {
        if char == '╭' {
            println!("found it");
        }
    }
    Ok(())
}
