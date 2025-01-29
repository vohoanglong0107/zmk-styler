use std::error::Error;

use zmk_styler::format;

fn main() -> Result<(), Box<dyn Error>> {
    format("glove80.keymap", "formatted.keymap")
}
