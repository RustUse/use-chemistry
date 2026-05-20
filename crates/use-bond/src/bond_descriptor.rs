use std::fmt;

use crate::BondValidationError;

/// A lightweight bond descriptor or reference label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BondDescriptor(String);

impl BondDescriptor {
    /// Creates a bond descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyDescriptor`] when `descriptor` is empty after trimming.
    pub fn new(descriptor: &str) -> Result<Self, BondValidationError> {
        let trimmed = descriptor.trim();
        if trimmed.is_empty() {
            Err(BondValidationError::EmptyDescriptor)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the descriptor text.
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

impl AsRef<str> for BondDescriptor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for BondDescriptor {
    type Error = BondValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for BondDescriptor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
