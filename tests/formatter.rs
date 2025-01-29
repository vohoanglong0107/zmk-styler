use std::fs;

use zmk_styler::format;

#[test]
fn test_formatter() {
    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(".");
    let in_file = "tests/glove80.keymap";
    let out_file = "tests/formatted.keymap";
    format(in_file, out_file).unwrap();
    let out = fs::read_to_string(out_file).unwrap();
    settings.bind(|| insta::assert_snapshot!("formatted", out))
}
