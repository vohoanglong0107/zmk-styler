use crate::lexer::Token;

pub(crate) struct Trivia {
    pub(crate) trivia_tokens: Vec<Token>,
}

impl Trivia {
    pub(crate) fn new(trivia_tokens: Vec<Token>) -> Self {
        Self { trivia_tokens }
    }
}
