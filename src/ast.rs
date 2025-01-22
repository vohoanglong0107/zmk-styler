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

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum PropertyValue {
    // This is only a formatter, we have no use for the value bool property
    Bool,
}
