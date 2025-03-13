use std::collections::HashSet;

use crate::{ast::AstNode, lexer::Token, source::Source, token_source::TokenSource};

pub(crate) struct FormatContext<'src> {
    pub(crate) source: &'src Source<'src>,
    pub(crate) trivia: TriviaFormatContext,
}

impl<'src> FormatContext<'src> {
    pub(crate) fn new(source: &'src Source<'src>, trivia: TokenSource) -> Self {
        Self {
            source,
            trivia: TriviaFormatContext::new(trivia),
        }
    }
}

pub(crate) struct TriviaFormatContext {
    token_source: TokenSource,
    formatted_trivia_indices: HashSet<usize>,
}

impl TriviaFormatContext {
    pub(crate) fn format_leading_trivia<T: AstNode>(&mut self, node: &T) -> Vec<Token> {
        let mut trivia = Vec::new();
        let Some(closest_preceding_trivia_index) = self.get_closest_preceding_trivia_for(node)
        else {
            return trivia;
        };
        let mut trivia_index = closest_preceding_trivia_index;
        while !self.formatted_trivia_indices.contains(&trivia_index)
            && self.token_source.tokens[trivia_index].is_trivia()
        {
            trivia.push(self.token_source.tokens[trivia_index].clone());
            self.formatted_trivia_indices.insert(trivia_index);
            if trivia_index == 0 {
                break;
            } else {
                trivia_index -= 1
            }
        }
        trivia.reverse();
        trivia
    }

    pub(crate) fn format_trailing_trivia<T: AstNode>(&mut self, node: &T) -> Vec<Token> {
        let mut trivia = Vec::new();
        let Some(closest_following_trivia_index) = self.get_closest_following_trivia_for(node)
        else {
            return trivia;
        };
        let mut trivia_index = closest_following_trivia_index;
        while !self.formatted_trivia_indices.contains(&trivia_index)
            && self.token_source.tokens[trivia_index].is_trivia()
            && !self.token_source.tokens[trivia_index].is_newline()
        {
            trivia.push(self.token_source.tokens[trivia_index].clone());
            self.formatted_trivia_indices.insert(trivia_index);
            if trivia_index == self.token_source.tokens.len() - 1 {
                break;
            } else {
                trivia_index += 1
            }
        }
        trivia
    }

    pub(crate) fn flush_trivia_after<T: AstNode>(&mut self, node: &T) -> Vec<Token> {
        let mut trivia = Vec::new();
        let Some(closest_following_trivia_index) = self.get_closest_following_trivia_for(node)
        else {
            return trivia;
        };
        let mut trivia_index = closest_following_trivia_index;
        while self.formatted_trivia_indices.contains(&trivia_index)
            && self.token_source.tokens[trivia_index].is_trivia()
        {
            trivia_index += 1
        }
        if trivia_index >= self.token_source.tokens.len() - 1 {
            return trivia;
        }
        while !self.formatted_trivia_indices.contains(&trivia_index)
            && self.token_source.tokens[trivia_index].is_trivia()
        {
            trivia.push(self.token_source.tokens[trivia_index].clone());
            self.formatted_trivia_indices.insert(trivia_index);
            if trivia_index == self.token_source.tokens.len() - 1 {
                break;
            } else {
                trivia_index += 1
            }
        }
        trivia
    }

    fn new(trivia: TokenSource) -> Self {
        Self {
            token_source: trivia,
            formatted_trivia_indices: HashSet::new(),
        }
    }

    fn get_closest_preceding_trivia_for<T: AstNode>(&self, node: &T) -> Option<usize> {
        let token_index = self
            .token_source
            .tokens
            .binary_search_by(|token| token.range.start().cmp(&node.range().start()))
            .expect("Node must start at boundary of a token");
        if token_index == 0 {
            None
        } else {
            Some(token_index - 1)
        }
    }

    fn get_closest_following_trivia_for<T: AstNode>(&self, node: &T) -> Option<usize> {
        let token_index = self
            .token_source
            .tokens
            .binary_search_by(|token| token.range.end().cmp(&node.range().end()))
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
