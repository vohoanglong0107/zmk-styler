use crate::{
    ast::{
        Identifier, Label, NodeAddress, NodeBody, NodeBodyEntry, NodeDefinition, NodeName,
        NonRootNodeIdentifier, RootNodeIdentifier,
    },
    lexer::TokenKind,
};

use super::{property::parse_property, utils::parse_list, ParseError, Parser};

pub(crate) fn parse_node(p: &mut Parser) -> Result<NodeDefinition, ParseError> {
    let start = p.start();
    let label = parse_label(p)?;
    let identifier = parse_node_identifier(p)?;
    let body = parse_node_body(p)?;
    Ok(NodeDefinition {
        label,
        identifier,
        body,
        range: p.end(start),
    })
}

fn parse_label(p: &mut Parser) -> Result<Option<Label>, ParseError> {
    if !p.nth_at(1, TokenKind::COLON) {
        return Ok(None);
    }
    let start = p.start();
    p.expect(TokenKind::NAME)?;
    p.expect(TokenKind::COLON)?;
    Ok(Some(Label {
        range: p.end(start),
    }))
}

fn parse_node_identifier(p: &mut Parser) -> Result<Identifier, ParseError> {
    let identifier = if p.at(TokenKind::ROOT) {
        Identifier::Root(parse_root_node_identifier(p)?)
    } else {
        Identifier::Other(parse_non_root_node_identifier(p)?)
    };
    Ok(identifier)
}

fn parse_root_node_identifier(p: &mut Parser) -> Result<RootNodeIdentifier, ParseError> {
    let start = p.start();
    p.expect(TokenKind::ROOT)?;
    Ok(RootNodeIdentifier {
        range: p.end(start),
    })
}

fn parse_non_root_node_identifier(p: &mut Parser) -> Result<NonRootNodeIdentifier, ParseError> {
    let start = p.start();
    let name = parse_node_name(p)?;
    let address = parse_node_address(p)?;
    Ok(NonRootNodeIdentifier {
        name,
        address,
        range: p.end(start),
    })
}

fn parse_node_name(p: &mut Parser) -> Result<NodeName, ParseError> {
    let start = p.start();
    p.expect(TokenKind::NAME)?;
    Ok(NodeName {
        range: p.end(start),
    })
}

fn parse_node_address(p: &mut Parser) -> Result<Option<NodeAddress>, ParseError> {
    if !p.at(TokenKind::AT) {
        return Ok(None);
    }
    let start = p.start();
    p.bump(TokenKind::AT);
    if p.at(TokenKind::INT) {
        p.bump(TokenKind::INT)
    } else {
        p.expect(TokenKind::NAME)?
    };
    Ok(Some(NodeAddress {
        range: p.end(start),
    }))
}

fn parse_node_body(p: &mut Parser) -> Result<NodeBody, ParseError> {
    let start = p.start();
    let l_curly = p.expect(TokenKind::L_CURLY)?;
    let entries = parse_list(p, parse_node_body_entry, TokenKind::R_CURLY, None)?;
    let r_curly = p.expect(TokenKind::R_CURLY)?;
    p.expect(TokenKind::SEMICOLON)?;
    Ok(NodeBody {
        l_curly,
        entries,
        r_curly,
        range: p.end(start),
    })
}

fn parse_node_body_entry(p: &mut Parser) -> Result<NodeBodyEntry, ParseError> {
    if is_at_node_property(p) {
        Ok(NodeBodyEntry::Property(parse_property(p)?))
    } else if is_at_child_node(p) {
        Ok(NodeBodyEntry::Node(parse_node(p)?))
    } else {
        return Err(ParseError::new(format!(
            "Expected a property or a child node, but found {}",
            p.current_token_kind()
        )));
    }
}

fn is_at_node_property(p: &Parser) -> bool {
    p.at(TokenKind::NAME) && (p.nth_at(1, TokenKind::SEMICOLON) || p.nth_at(1, TokenKind::EQUAL))
}

fn is_at_child_node(p: &Parser) -> bool {
    p.at(TokenKind::NAME) || p.at(TokenKind::ROOT)
}
