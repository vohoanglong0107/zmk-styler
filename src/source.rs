use std::{fmt::Display, ops::Index};

pub(crate) struct Source<'src> {
    data: &'src str,
}

/// Range of a text object, exclusive
#[derive(Clone, Copy, Debug)]
pub(crate) struct SourceRange {
    start: SourceIndex,
    end: SourceIndex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) struct SourceIndex {
    value: usize,
}

impl<'src> Source<'src> {
    pub(crate) fn new(src: &'src str) -> Self {
        Self { data: src }
    }

    pub(crate) fn is_eof(&self, index: SourceIndex) -> bool {
        index.value >= self.data.len()
    }

    pub(crate) fn get(&self, index: SourceIndex) -> Option<&u8> {
        self.data.as_bytes().get(index.value)
    }
}

impl Index<SourceRange> for Source<'_> {
    type Output = [u8];

    fn index(&self, index: SourceRange) -> &Self::Output {
        self.data
            .as_bytes()
            .index(index.start.value..index.end.value)
    }
}

impl Index<SourceIndex> for Source<'_> {
    type Output = u8;

    fn index(&self, index: SourceIndex) -> &Self::Output {
        self.data.as_bytes().index(index.value)
    }
}

impl SourceRange {
    pub(crate) fn new(start: SourceIndex, end: SourceIndex) -> Self {
        SourceRange { start, end }
    }

    // For when we don't need to use value of the range, like the EOF token
    // Max value to help with binary search
    pub(crate) fn null() -> Self {
        SourceRange {
            start: SourceIndex { value: usize::MAX },
            end: SourceIndex { value: usize::MAX },
        }
    }

    pub(crate) fn start(&self) -> SourceIndex {
        self.start
    }

    pub(crate) fn end(&self) -> SourceIndex {
        self.end
    }
}

impl From<SourceRange> for core::ops::Range<usize> {
    fn from(value: SourceRange) -> Self {
        Self {
            start: value.start.value,
            end: value.end.value,
        }
    }
}

impl SourceIndex {
    pub(crate) fn increment(&self) -> Self {
        Self {
            value: self.value + 1,
        }
    }
}

impl Display for Source<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.data)
    }
}

impl Display for SourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

impl Display for SourceIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
