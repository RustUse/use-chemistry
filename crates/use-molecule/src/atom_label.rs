use std::fmt;

use use_chemical_formula::ElementSymbol;

use crate::MoleculeValidationError;

/// A validated atom label with a basic element-symbol shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AtomLabel(ElementSymbol);

impl AtomLabel {
    /// Creates an atom label.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyAtomLabel`] when `label` is empty after trimming, or
    /// [`MoleculeValidationError::InvalidAtomLabel`] when the label does not match the supported
    /// element-symbol shape.
    pub fn new(label: &str) -> Result<Self, MoleculeValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            return Err(MoleculeValidationError::EmptyAtomLabel);
        }

        ElementSymbol::new(trimmed)
            .map(Self)
            .map_err(|_| MoleculeValidationError::InvalidAtomLabel(trimmed.to_owned()))
    }

    /// Returns the atom label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the wrapped formula element-symbol value.
    #[must_use]
    pub const fn as_element_symbol(&self) -> &ElementSymbol {
        &self.0
    }
}

impl AsRef<str> for AtomLabel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for AtomLabel {
    type Error = MoleculeValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for AtomLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
