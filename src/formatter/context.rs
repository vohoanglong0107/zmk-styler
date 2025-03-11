use crate::{ast::AstNode, lexer::Token, source::Source, trivia::Trivia};

pub(crate) struct FormatContext<'src> {
    pub(crate) source: &'src Source<'src>,
    pub(crate) trivia: TriviaFormatContext,
}

impl<'src> FormatContext<'src> {
    pub(crate) fn new(source: &'src Source<'src>, trivia: Trivia) -> Self {
        Self {
            source,
            trivia: TriviaFormatContext::new(trivia),
        }
    }
}

pub(crate) struct TriviaFormatContext {
    trivia: Trivia,
    formatted_trivia_index: Option<usize>,
}

impl TriviaFormatContext {
    pub(crate) fn unformatted_trivia<T: AstNode>(&self, node: &T) -> Vec<Token> {
        let mut trivia = Vec::new();
        let mut closest_preceding_trivia_index = self.get_closest_preceding_trivia_for(node);
        while self.is_unformatted_trivia(closest_preceding_trivia_index) {
            trivia.push(self.trivia.trivia_tokens[closest_preceding_trivia_index].clone());
            if closest_preceding_trivia_index == 0 {
                break;
            } else {
                closest_preceding_trivia_index -= 1
            }
        }
        trivia.reverse();
        trivia
    }

    pub(crate) fn formatted_up_to<T: AstNode>(&mut self, node: &T) {
        let closest_preceding_trivia_index = self.get_closest_preceding_trivia_for(node);
        self.formatted_trivia_index = Some(closest_preceding_trivia_index)
    }

    fn new(trivia: Trivia) -> Self {
        Self {
            trivia,
            formatted_trivia_index: None,
        }
    }

    fn get_closest_preceding_trivia_for<T: AstNode>(&self, node: &T) -> usize {
        self.trivia
            .trivia_tokens
            .binary_search_by(|token| token.range.start().cmp(&node.range().start()))
            .map_or_else(
                |trivia_index| trivia_index,
                |_| panic!("Comments must not contain non trivia tokens."),
            )
    }

    fn is_unformatted_trivia(&self, trivia_index: usize) -> bool {
        match self.formatted_trivia_index {
            None => true,
            Some(formatted_index) => trivia_index > formatted_index,
        }
    }
}
