#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) indent_width: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { indent_width: 4 }
    }
}
