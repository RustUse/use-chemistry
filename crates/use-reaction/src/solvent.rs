use std::fmt;

use crate::ReactionValidationError;

/// A lightweight solvent descriptor.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Solvent(String);

impl Solvent {
    /// Creates a solvent descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptySolventLabel`] when `label` is empty after
    /// trimming.
    pub fn new(label: &str) -> Result<Self, ReactionValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(ReactionValidationError::EmptySolventLabel)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the solvent descriptor text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the descriptor and returns the owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for Solvent {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for Solvent {
    type Error = ReactionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for Solvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
