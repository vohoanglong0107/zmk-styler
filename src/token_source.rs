use crate::lexer::Token;

pub(crate) struct TokenSource {
    pub(crate) tokens: Vec<Token>,
}

impl TokenSource {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}
