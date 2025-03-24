use crate::lexer::TokenKind;

use super::Parser;

// TODO: recovery from ill-formed node
pub(super) fn parse_list<F>(
    p: &mut Parser,
    element_parser: F,
    end: TokenKind,
    separator: Option<TokenKind>,
) where
    F: Fn(&mut Parser),
{
    let mut is_first = true;
    loop {
        if p.at(end) {
            break;
        }
        if let Some(separator) = separator {
            if !is_first {
                p.expect(separator);
            } else {
                is_first = false;
            }
        };
        element_parser(p);
    }
}
