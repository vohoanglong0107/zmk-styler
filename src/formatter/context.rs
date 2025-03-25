use crate::{
    lexer::Token,
    source::{Source, SourceRange},
    token_source::TokenSource,
};

pub(crate) struct FormatContext<'src> {
    pub(crate) source: &'src Source<'src>,
    pub(crate) trivia: TriviaFormatContext<'src>,
}

impl<'src> FormatContext<'src> {
    pub(crate) fn new(source: &'src Source<'src>, trivia: &'src TokenSource) -> Self {
        Self {
            source,
            trivia: TriviaFormatContext::new(trivia),
        }
    }
}

pub(crate) struct TriviaFormatContext<'src> {
    token_source: &'src TokenSource,
}

impl<'src> TriviaFormatContext<'src> {
    pub(crate) fn leading_trivia(&self, range: SourceRange) -> Vec<Token> {
        let mut trivia = Vec::new();
        let Some(closest_preceding_trivia_index) = self.get_closest_preceding_trivia_for(range)
        else {
            return trivia;
        };
        let mut trivia_index = closest_preceding_trivia_index;
        while self.token_source.tokens[trivia_index].is_trivia() {
            trivia.push(self.token_source.tokens[trivia_index].clone());
            if trivia_index == 0 {
                break;
            } else {
                trivia_index -= 1
            }
        }
        trivia.reverse();
        if trivia_index == 0 {
            return trivia;
        }
        // Remove previous's node trailing trivia
        let first_new_line_index = trivia.iter().position(|trivia| trivia.is_newline());
        match first_new_line_index {
            None => trivia,
            Some(index) => {
                let (_, trivia) = trivia.split_at(index);
                trivia.to_vec()
            }
        }
    }

    pub(crate) fn trailing_trivia(&self, range: SourceRange) -> Vec<Token> {
        let mut trivia = Vec::new();
        let Some(closest_following_trivia_index) = self.get_closest_following_trivia_for(range)
        else {
            return trivia;
        };
        let mut trivia_index = closest_following_trivia_index;
        while self.token_source.tokens[trivia_index].is_trivia()
            && !self.token_source.tokens[trivia_index].is_newline()
        {
            trivia.push(self.token_source.tokens[trivia_index].clone());
            if trivia_index == self.token_source.tokens.len() - 1 {
                break;
            } else {
                trivia_index += 1
            }
        }
        trivia
    }

    fn new(trivia: &'src TokenSource) -> Self {
        Self {
            token_source: trivia,
        }
    }

    fn get_closest_preceding_trivia_for(&self, range: SourceRange) -> Option<usize> {
        let token_index = self
            .token_source
            .tokens
            .binary_search_by(|token| token.range.start().cmp(&range.start()))
            .expect("Node must start at boundary of a token");
        if token_index == 0 {
            None
        } else {
            Some(token_index - 1)
        }
    }

    fn get_closest_following_trivia_for(&self, range: SourceRange) -> Option<usize> {
        let token_index = self
            .token_source
            .tokens
            .binary_search_by(|token| token.range.end().cmp(&range.end()))
            .expect("Node must end at boundary of a token");
        if token_index >= self.token_source.tokens.len() - 1
            || self.token_source.tokens[token_index].is_eof()
        {
            None
        } else {
            Some(token_index + 1)
        }
    }
}
