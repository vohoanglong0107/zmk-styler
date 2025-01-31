use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Node {
    // Zephyr mentioned that one node can have multiple labels, but I found no document for that
    // ref: https://docs.zephyrproject.org/latest/build/dts/intro-syntax-structure.html#nodes
    pub(crate) label: Option<String>,
    pub(crate) name: String,
    pub(crate) address: Option<String>,
    pub(crate) children: Vec<Node>,
    pub(crate) properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Property {
    pub(crate) name: String,
    pub(crate) value: PropertyValue,
}

/// Property values may be defined as an array of 32-bit integer cells, as null-terminated strings, as bytestrings or a combination of these.
/// https://devicetree-specification.readthedocs.io/en/latest/chapter6-source-language.html
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PropertyValue {
    // This is only a formatter, we have no use for the value bool property
    Bool,
    Values(PropertyValues),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PropertyValues(Vec<NonBoolPropertyValue>);

impl From<Vec<NonBoolPropertyValue>> for PropertyValues {
    fn from(value: Vec<NonBoolPropertyValue>) -> Self {
        Self(value)
    }
}

impl IntoIterator for PropertyValues {
    type Item = NonBoolPropertyValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NonBoolPropertyValue {
    Reference(ReferenceValue),
    Array(ArrayValue),
    String(StringValue),
    ByteString(ByteStringValue),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ArrayValue(Vec<ArrayCell>);

impl From<Vec<ArrayCell>> for ArrayValue {
    fn from(value: Vec<ArrayCell>) -> Self {
        Self(value)
    }
}

impl IntoIterator for ArrayValue {
    type Item = ArrayCell;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ArrayCell {
    Int(String), // Technically we should use i32, but we have no use for the value
    Reference(ReferenceValue),
    Path(ReferencePath),
    Expression(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct StringValue(String);

impl From<&str> for StringValue {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Display for StringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ByteStringValue(Vec<[ByteStringCharacter; 2]>);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ByteStringCharacter(char);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReferenceValue(String);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReferencePath(String);
