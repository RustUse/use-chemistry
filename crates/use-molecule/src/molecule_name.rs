use std::fmt;

use crate::MoleculeValidationError;

/// A validated molecule name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoleculeName(String);

impl MoleculeName {
    /// Creates a molecule name.
    ///
    /// # Errors
    ///
    /// Returns [`MoleculeValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn new(name: &str) -> Result<Self, MoleculeValidationError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(MoleculeValidationError::EmptyName)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the molecule name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the name and returns the owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for MoleculeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for MoleculeName {
    type Error = MoleculeValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for MoleculeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
