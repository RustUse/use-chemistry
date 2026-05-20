use std::fmt;

use crate::{AtomIndex, MoleculeValidationError};

/// A simple connection between explicit atoms.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AtomConnection {
    /// The source atom index.
    pub from: AtomIndex,
    /// The target atom index.
    pub to: AtomIndex,
    /// Optional connection order. This crate does not assign bonding semantics to the value.
    pub order: Option<u8>,
}

impl AtomConnection {
    /// Creates an atom connection.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::SelfConnection`] when `from` equals `to`, or
    /// [`MoleculeValidationError::ZeroConnectionOrder`] when `order` is `Some(0)`.
    pub fn new(
        from: AtomIndex,
        to: AtomIndex,
        order: Option<u8>,
    ) -> Result<Self, MoleculeValidationError> {
        if from == to {
            return Err(MoleculeValidationError::SelfConnection { index: from.get() });
        }

        if order == Some(0) {
            return Err(MoleculeValidationError::ZeroConnectionOrder);
        }

        Ok(Self { from, to, order })
    }

    pub(crate) fn validate_indices(
        self,
        atom_count: usize,
    ) -> Result<Self, MoleculeValidationError> {
        for index in [self.from, self.to] {
            if index.get() >= atom_count {
                return Err(MoleculeValidationError::InvalidConnectionIndex {
                    index: index.get(),
                    atom_count,
                });
            }
        }

        Ok(self)
    }
}

impl fmt::Display for AtomConnection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.order {
            Some(order) => write!(formatter, "{}-{}({order})", self.from, self.to),
            None => write!(formatter, "{}-{}", self.from, self.to),
        }
    }
}
