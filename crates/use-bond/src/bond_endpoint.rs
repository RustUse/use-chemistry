use std::fmt;

use crate::BondValidationError;

/// A validated atom endpoint reference for a bond.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BondEndpoint(String);

impl BondEndpoint {
    /// Creates a bond endpoint label.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyEndpointLabel`] when `label` is empty after trimming.
    pub fn new(label: &str) -> Result<Self, BondValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(BondValidationError::EmptyEndpointLabel)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the endpoint label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the endpoint and returns the owned label.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for BondEndpoint {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for BondEndpoint {
    type Error = BondValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for BondEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
