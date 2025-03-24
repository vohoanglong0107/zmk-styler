#![allow(clippy::manual_map)]

use std::rc::Rc;

use crate::{
    lexer::{Token, TokenKind},
    source::SourceRange,
};

pub(crate) trait AstNode: Sized {
    fn cast(syntax: &SyntaxNode) -> Option<Self>;
    fn range(&self) -> SourceRange;
}

type SyntaxResult<T> = Result<T, ()>;

#[derive(Debug)]
pub(crate) struct Document {
    syntax: SyntaxNode,
}

impl Document {
    pub(crate) fn statements(&self) -> impl IntoIterator<Item = Statement> + '_ {
        get_child_nodes(&self.syntax)
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Node(NodeDefinition),
}

#[derive(Debug)]
pub(crate) struct NodeDefinition {
    syntax: SyntaxNode,
}

impl NodeDefinition {
    // Zephyr mentioned that one node can have multiple labels, but I found no document for that
    // ref: https://docs.zephyrproject.org/latest/build/dts/intro-syntax-structure.html#nodes
    pub(crate) fn label(&self) -> Option<Label> {
        get_child_node(&self.syntax).ok()
    }

    // name@address, or "/" for root node
    pub(crate) fn identifier(&self) -> SyntaxResult<NodeIdentifier> {
        get_child_node(&self.syntax)
    }

    pub(crate) fn body(&self) -> SyntaxResult<NodeBody> {
        get_child_node(&self.syntax)
    }
}

#[derive(Debug)]
pub(crate) struct Label {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) enum NodeIdentifier {
    Root(RootNodeIdentifier),
    NonRoot(NonRootNodeIdentifier),
}

#[derive(Debug)]
pub(crate) struct NonRootNodeIdentifier {
    syntax: SyntaxNode,
}

impl NonRootNodeIdentifier {
    pub(crate) fn name(&self) -> SyntaxResult<NodeName> {
        get_child_node(&self.syntax)
    }

    pub(crate) fn address(&self) -> Option<NodeAddress> {
        get_child_node(&self.syntax).ok()
    }
}

#[derive(Debug)]
pub(crate) struct NodeName {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) struct NodeAddress {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) struct RootNodeIdentifier {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) struct NodeBody {
    syntax: SyntaxNode,
}

impl NodeBody {
    pub(crate) fn entries(&self) -> SyntaxResult<NodeBodyEntries> {
        get_child_node(&self.syntax)
    }

    pub(crate) fn l_curly(&self) -> SyntaxResult<Token> {
        get_token(&self.syntax, TokenKind::L_CURLY)
    }

    pub(crate) fn r_curly(&self) -> SyntaxResult<Token> {
        get_token(&self.syntax, TokenKind::R_CURLY)
    }
}

pub(crate) struct NodeBodyEntries {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) enum NodeBodyEntry {
    Node(NodeDefinition),
    Property(PropertyDefinition),
}

#[derive(Debug)]
pub(crate) enum PropertyDefinition {
    Bool(BoolPropertyDefinition),
    NonBool(NonBoolPropertyDefinition),
}

#[derive(Debug)]
pub(crate) struct BoolPropertyDefinition {
    syntax: SyntaxNode,
}

impl BoolPropertyDefinition {
    pub(crate) fn name(&self) -> SyntaxResult<PropertyName> {
        get_child_node(&self.syntax)
    }
}

#[derive(Debug)]
pub(crate) struct NonBoolPropertyDefinition {
    syntax: SyntaxNode,
}

impl NonBoolPropertyDefinition {
    pub(crate) fn name(&self) -> SyntaxResult<PropertyName> {
        get_child_node(&self.syntax)
    }

    pub(crate) fn values(&self) -> SyntaxResult<PropertyValues> {
        get_child_node(&self.syntax)
    }
}

#[derive(Debug)]
pub(crate) struct PropertyName {
    syntax: SyntaxNode,
}

/// Property values may be defined as an array of 32-bit integer cells, as null-terminated strings, as bytestrings or a combination of these.
/// https://devicetree-specification.readthedocs.io/en/latest/chapter6-source-language.html
#[derive(Debug)]
pub(crate) struct PropertyValues {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) enum PropertyValue {
    Array(ArrayValue),
    String(StringValue),
}

#[derive(Debug)]
pub(crate) struct ArrayValue {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) enum ArrayCell {
    Int(IntCell),
}

#[derive(Debug)]
pub(crate) struct IntCell {
    syntax: SyntaxNode,
}

#[derive(Debug)]
pub(crate) struct StringValue {
    syntax: SyntaxNode,
}

impl AstNode for Document {
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::Document) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }

    fn range(&self) -> SourceRange {
        self.syntax.range
    }
}

impl AstNode for Statement {
    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(node) = NodeDefinition::cast(syntax) {
            Some(Self::Node(node))
        } else {
            None
        }
    }
    fn range(&self) -> SourceRange {
        match self {
            Self::Node(node) => node.range(),
        }
    }
}

impl AstNode for NodeDefinition {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NodeDefinition) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for Label {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::Label) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeIdentifier {
    fn range(&self) -> SourceRange {
        match self {
            Self::Root(root) => root.range(),
            Self::NonRoot(iden) => iden.range(),
        }
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(identifier) = RootNodeIdentifier::cast(syntax) {
            Some(Self::Root(identifier))
        } else if let Some(identifier) = NonRootNodeIdentifier::cast(syntax) {
            Some(Self::NonRoot(identifier))
        } else {
            None
        }
    }
}

impl AstNode for NonRootNodeIdentifier {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NonRootNodeIdentifier) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeName {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NodeName) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeAddress {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NodeAddress) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for RootNodeIdentifier {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::RootNodeIdentifier) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeBody {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NodeBody) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeBodyEntries {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NodeBodyEntries) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NodeBodyEntry {
    fn range(&self) -> SourceRange {
        match self {
            Self::Node(node) => node.range(),
            Self::Property(prop) => prop.range(),
        }
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(identifier) = NodeDefinition::cast(syntax) {
            Some(Self::Node(identifier))
        } else if let Some(identifier) = PropertyDefinition::cast(syntax) {
            Some(Self::Property(identifier))
        } else {
            None
        }
    }
}

impl AstNode for PropertyDefinition {
    fn range(&self) -> SourceRange {
        match self {
            Self::Bool(prop) => prop.range(),
            Self::NonBool(prop) => prop.range(),
        }
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(identifier) = BoolPropertyDefinition::cast(syntax) {
            Some(Self::Bool(identifier))
        } else if let Some(identifier) = NonBoolPropertyDefinition::cast(syntax) {
            Some(Self::NonBool(identifier))
        } else {
            None
        }
    }
}

impl AstNode for BoolPropertyDefinition {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::BoolPropertyDefinition) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for NonBoolPropertyDefinition {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::NonBoolPropertyDefinition) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for PropertyName {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::PropertyName) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for PropertyValues {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::PropertyValues) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for PropertyValue {
    fn range(&self) -> SourceRange {
        match self {
            Self::Array(a) => a.range(),
            Self::String(s) => s.range(),
        }
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(identifier) = ArrayValue::cast(syntax) {
            Some(Self::Array(identifier))
        } else if let Some(identifier) = StringValue::cast(syntax) {
            Some(Self::String(identifier))
        } else {
            None
        }
    }
}

impl AstNode for ArrayValue {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::ArrayValue) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for ArrayCell {
    fn range(&self) -> SourceRange {
        match self {
            Self::Int(i) => i.range(),
        }
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if let Some(identifier) = IntCell::cast(syntax) {
            Some(Self::Int(identifier))
        } else {
            None
        }
    }
}

impl AstNode for IntCell {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::IntCell) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl AstNode for StringValue {
    fn range(&self) -> SourceRange {
        self.syntax.range
    }

    fn cast(syntax: &SyntaxNode) -> Option<Self> {
        if matches!(syntax.kind, SyntaxKind::StringValue) {
            Some(Self {
                syntax: syntax.clone(),
            })
        } else {
            None
        }
    }
}

impl IntoIterator for NodeBodyEntries {
    type Item = NodeBodyEntry;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        get_child_nodes(&self.syntax).into_iter()
    }
}

impl IntoIterator for PropertyValues {
    type Item = PropertyValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        get_child_nodes(&self.syntax).into_iter()
    }
}

impl IntoIterator for ArrayValue {
    type Item = ArrayCell;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        get_child_nodes(&self.syntax).into_iter()
    }
}

pub(crate) type SyntaxNode = Rc<SyntaxNodeData>;

#[derive(Debug)]
pub(crate) struct SyntaxNodeData {
    kind: SyntaxKind,
    children: Vec<SyntaxNodeChild>,
    range: SourceRange,
}

pub(crate) struct SyntaxNodeBuilder {
    kind: Option<SyntaxKind>,
    children: Vec<SyntaxNodeChild>,
    range: Option<SourceRange>,
}

impl SyntaxNodeBuilder {
    pub(crate) fn new() -> Self {
        Self {
            kind: None,
            children: Vec::new(),
            range: None,
        }
    }

    pub(crate) fn push_node(&mut self, node: SyntaxNode) {
        self.children.push(SyntaxNodeChild::Tree(node));
    }

    pub(crate) fn push_token(&mut self, token: Token) {
        self.children.push(SyntaxNodeChild::Token(token));
    }

    pub(crate) fn kind(&mut self, kind: SyntaxKind) {
        self.kind = Some(kind)
    }

    pub(crate) fn range(&mut self, range: SourceRange) {
        self.range = Some(range)
    }

    pub(crate) fn build(self) -> SyntaxNode {
        Rc::new(SyntaxNodeData {
            kind: self.kind.unwrap(),
            children: self.children,
            range: self.range.unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) enum SyntaxNodeChild {
    Token(Token),
    Tree(SyntaxNode),
}

impl SyntaxNodeChild {
    fn as_token(&self) -> Option<&Token> {
        match self {
            SyntaxNodeChild::Token(token) => Some(token),
            _ => None,
        }
    }

    fn as_node(&self) -> Option<&SyntaxNode> {
        match self {
            SyntaxNodeChild::Tree(node) => Some(node),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum SyntaxKind {
    Document,
    NodeDefinition,
    Label,
    RootNodeIdentifier,
    NonRootNodeIdentifier,
    NodeName,
    NodeAddress,
    NodeBody,
    NodeBodyEntries,
    BoolPropertyDefinition,
    NonBoolPropertyDefinition,
    PropertyName,
    PropertyValues,
    ArrayValue,
    IntCell,
    StringValue,
}

fn get_child_nodes<'a, T: AstNode + 'a>(syntax: &'a SyntaxNode) -> Vec<T> {
    syntax
        .children
        .iter()
        .filter_map(SyntaxNodeChild::as_node)
        .filter_map(T::cast)
        .collect()
}

fn get_child_node<T: AstNode>(syntax: &SyntaxNode) -> SyntaxResult<T> {
    syntax
        .children
        .iter()
        .filter_map(SyntaxNodeChild::as_node)
        .find_map(T::cast)
        .ok_or(())
}

fn get_token(syntax: &SyntaxNode, kind: TokenKind) -> SyntaxResult<Token> {
    syntax
        .children
        .iter()
        .filter_map(SyntaxNodeChild::as_token)
        .find(|token| token.kind == kind)
        .cloned()
        .ok_or(())
}
