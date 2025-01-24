use std::{error::Error, fmt::Display};

use node::parse_node;

use crate::ast::Node;

pub(crate) mod node;
pub(crate) mod property;

#[derive(Debug)]
pub(crate) struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl Error for ParseError {}

pub(crate) fn parse(input: &str) -> Result<Node, ParseError> {
    let (rest, node) = match parse_node(input) {
        Ok((rest, node)) => (rest, node),
        Err(e) => {
            println!("Err: {e}");
            return Err(ParseError);
        }
    };
    if rest.is_empty() {
        Ok(node)
    } else {
        println!("Can't parse entire file. Unparsed section: {rest}");
        Err(ParseError)
    }
}
