use std::fmt;

/// A zero-based atom index into a molecule's explicit atom list.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AtomIndex(usize);

impl AtomIndex {
    /// Creates an atom index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based index value.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

impl From<usize> for AtomIndex {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for AtomIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A count of explicit atoms in a molecule.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AtomCount(usize);

impl AtomCount {
    /// Creates an atom count.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count)
    }

    /// Returns the count value.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns `true` when the count is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<usize> for AtomCount {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for AtomCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
