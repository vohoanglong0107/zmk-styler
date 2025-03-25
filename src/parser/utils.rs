use crate::lexer::TokenKind;

use super::Parser;

// TODO: recovery from ill-formed node
pub(super) fn parse_list<F, C, R>(
    p: &mut Parser,
    element_parser: F,
    is_at_element: C,
    end: TokenKind,
    separator: Option<TokenKind>,
    is_recovered: R,
) where
    F: Fn(&mut Parser),
    C: Fn(&Parser) -> bool,
    R: Fn(&Parser) -> bool,
{
    let mut is_first = true;
    loop {
        if p.at(end) || p.at(TokenKind::EOF) {
            break;
        }
        if let Some(separator) = separator {
            if !is_first {
                p.expect(separator);
            } else {
                is_first = false;
            }
        };
        if is_at_element(p) {
            element_parser(p);
        } else {
            while !is_recovered(p) && !p.at(TokenKind::EOF) {
                p.bump_any();
            }
        }
    }
}
