use std::fmt;

use crate::BondValidationError;

/// A validated bond participant reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BondParticipant(String);

impl BondParticipant {
    /// Creates a bond participant label.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::EmptyParticipantLabel`] when `label` is empty after trimming.
    pub fn new(label: &str) -> Result<Self, BondValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(BondValidationError::EmptyParticipantLabel)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the participant label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the participant and returns the owned label.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for BondParticipant {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for BondParticipant {
    type Error = BondValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for BondParticipant {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
