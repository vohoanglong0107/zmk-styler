use crate::{lexer::Token, source::SourceRange};

pub(crate) trait AstNode {
    fn range(&self) -> SourceRange;
}

#[derive(Debug)]
pub(crate) struct Document {
    pub(crate) statements: Vec<Statement>,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) enum Statement {
    Node(NodeDefinition),
}

#[derive(Debug)]
pub(crate) struct NodeDefinition {
    // Zephyr mentioned that one node can have multiple labels, but I found no document for that
    // ref: https://docs.zephyrproject.org/latest/build/dts/intro-syntax-structure.html#nodes
    pub(crate) label: Option<Label>,
    // name@address, or "/" for root node
    pub(crate) identifier: Identifier,
    pub(crate) body: NodeBody,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct Label {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) enum Identifier {
    Root(RootNodeIdentifier),
    Other(NonRootNodeIdentifier),
}

#[derive(Debug)]
pub(crate) struct NonRootNodeIdentifier {
    pub(crate) name: NodeName,
    pub(crate) address: Option<NodeAddress>,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct NodeName {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct NodeAddress {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct RootNodeIdentifier {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct NodeBody {
    pub(crate) l_curly: Token,
    pub(crate) entries: Vec<NodeBodyEntry>,
    pub(crate) r_curly: Token,
    pub(crate) range: SourceRange,
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
    pub(crate) name: PropertyName,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct NonBoolPropertyDefinition {
    pub(crate) name: PropertyName,
    pub(crate) values: PropertyValues,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct PropertyName {
    pub(crate) range: SourceRange,
}

/// Property values may be defined as an array of 32-bit integer cells, as null-terminated strings, as bytestrings or a combination of these.
/// https://devicetree-specification.readthedocs.io/en/latest/chapter6-source-language.html
#[derive(Debug)]
pub(crate) struct PropertyValues {
    pub(crate) values: Vec<PropertyValue>,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) enum PropertyValue {
    Reference(ReferenceValue),
    Array(ArrayValue),
    String(StringValue),
    ByteString(ByteStringValue),
}

#[derive(Debug)]
pub(crate) struct ArrayValue {
    pub(crate) cells: Vec<ArrayCell>,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) enum ArrayCell {
    Int(IntValue),
    Reference(ReferenceValue),
    Path(ReferencePath),
    Expression(Expression),
}

#[derive(Debug)]
pub(crate) struct IntValue {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct StringValue {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct ByteStringValue {
    pub(crate) value: Vec<[ByteStringCharacter; 2]>,
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct ByteStringCharacter {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct ReferenceValue {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct ReferencePath {
    pub(crate) range: SourceRange,
}

#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) range: SourceRange,
}

impl AstNode for Document {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for Statement {
    fn range(&self) -> SourceRange {
        match self {
            Self::Node(node) => node.range(),
        }
    }
}

impl AstNode for NodeDefinition {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for Label {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for Identifier {
    fn range(&self) -> SourceRange {
        match self {
            Self::Root(root) => root.range(),
            Self::Other(iden) => iden.range(),
        }
    }
}

impl AstNode for NonRootNodeIdentifier {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for NodeName {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for NodeAddress {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for RootNodeIdentifier {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for NodeBody {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for NodeBodyEntry {
    fn range(&self) -> SourceRange {
        match self {
            Self::Node(node) => node.range(),
            Self::Property(prop) => prop.range(),
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
}

impl AstNode for BoolPropertyDefinition {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for NonBoolPropertyDefinition {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for PropertyName {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for PropertyValues {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for PropertyValue {
    fn range(&self) -> SourceRange {
        match self {
            Self::Reference(r) => r.range(),
            Self::Array(a) => a.range(),
            Self::String(s) => s.range(),
            Self::ByteString(s) => s.range(),
        }
    }
}

impl AstNode for ArrayValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for ArrayCell {
    fn range(&self) -> SourceRange {
        match self {
            Self::Int(i) => i.range(),
            Self::Reference(r) => r.range(),
            Self::Path(p) => p.range(),
            Self::Expression(e) => e.range(),
        }
    }
}

impl AstNode for IntValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for StringValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for ByteStringValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for ByteStringCharacter {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for ReferenceValue {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for ReferencePath {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl AstNode for Expression {
    fn range(&self) -> SourceRange {
        self.range
    }
}

impl IntoIterator for PropertyValues {
    type Item = PropertyValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl IntoIterator for ArrayValue {
    type Item = ArrayCell;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}
