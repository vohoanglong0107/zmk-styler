use crate::{lexer::Token, source::SourceIndex};

pub(crate) struct TokenSource {
    pub(crate) tokens: Vec<Token>,
}

impl TokenSource {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub(crate) fn get_line_number(&self, source_index: SourceIndex) -> u32 {
        let token_index = self
            .tokens
            .binary_search_by(|token| token.range.start().cmp(&source_index))
            .unwrap_or_else(|index| index);
        let tokens = &self.tokens[0..token_index];
        let num_new_lines: u32 = tokens
            .iter()
            .filter(|token| token.is_newline())
            .count()
            .try_into()
            .unwrap();
        num_new_lines + 1
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::Lexer, source::Source};

    use super::TokenSource;

    #[test]
    fn get_line_number() {
        let source = Source::new(
            r#"/* Glove80 system behavior & macros */ / {
    behaviors {
        // For the "layer" key, it'd nice to be able to use it as either a shift or a toggle.
        lower: lower {
            compatible = "zmk,behavior-tap-dance";
            label = "LAYER_TAP_DANCE";
            #binding-cells = <0>;
            tapping-term-ms = <200>;
        };
    };
};
        "#,
        );
        let mut lexer = Lexer::new(&source);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            if token.is_eof() {
                break;
            }
            tokens.push(token);
        }
        let token_source = TokenSource::new(tokens.clone());
        let compatible_token = tokens
            .iter()
            .find(|token| std::str::from_utf8(&source[token.range]).unwrap() == "compatible")
            .unwrap();
        assert_eq!(
            token_source.get_line_number(compatible_token.range.start()),
            5
        );
    }
}
