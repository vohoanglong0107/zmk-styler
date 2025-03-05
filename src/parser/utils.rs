use crate::{ast::AstNode, lexer::TokenKind};

use super::{ParseError, Parser};

// TODO: recovery from ill-formed node
pub(super) fn parse_list<T, F>(
    p: &mut Parser,
    element_parser: F,
    end: TokenKind,
    separator: Option<TokenKind>,
) -> Result<Vec<T>, ParseError>
where
    T: AstNode,
    F: Fn(&mut Parser) -> Result<T, ParseError>,
{
    let mut elements = Vec::new();

    let mut is_first = true;
    loop {
        if p.at(end) {
            break;
        }
        if let Some(separator) = separator {
            if !is_first {
                p.expect(separator)?;
            } else {
                is_first = false;
            }
        };
        elements.push(element_parser(p)?);
    }
    Ok(elements)
}
