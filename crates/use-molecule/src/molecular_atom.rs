use std::fmt;

use crate::{AtomLabel, MoleculeValidationError};

/// A validated atom identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MolecularAtomId(String);

impl MolecularAtomId {
    /// Creates an atom identifier.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyAtomId`] when `id` is empty after trimming.
    pub fn new(id: &str) -> Result<Self, MoleculeValidationError> {
        let trimmed = id.trim();
        if trimmed.is_empty() {
            Err(MoleculeValidationError::EmptyAtomId)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the atom identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns the owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for MolecularAtomId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for MolecularAtomId {
    type Error = MoleculeValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for MolecularAtomId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An explicit atom entry in a molecule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MolecularAtom {
    label: AtomLabel,
    id: Option<MolecularAtomId>,
}

impl MolecularAtom {
    /// Creates a molecular atom from an atom label.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError`] when `label` is empty or does not match the supported
    /// element-symbol shape.
    pub fn new(label: &str) -> Result<Self, MoleculeValidationError> {
        Ok(Self {
            label: AtomLabel::new(label)?,
            id: None,
        })
    }

    /// Returns the atom label.
    #[must_use]
    pub const fn label(&self) -> &AtomLabel {
        &self.label
    }

    /// Returns the optional atom identifier.
    #[must_use]
    pub const fn id(&self) -> Option<&MolecularAtomId> {
        self.id.as_ref()
    }

    /// Sets the atom identifier from a validated value.
    #[must_use]
    pub fn with_id(mut self, id: MolecularAtomId) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the atom identifier after validation.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyAtomId`] when `id` is empty after trimming.
    pub fn try_with_id(self, id: &str) -> Result<Self, MoleculeValidationError> {
        Ok(self.with_id(MolecularAtomId::new(id)?))
    }
}

impl fmt::Display for MolecularAtom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.id {
            Some(id) => write!(formatter, "{}#{id}", self.label),
            None => write!(formatter, "{}", self.label),
        }
    }
}
