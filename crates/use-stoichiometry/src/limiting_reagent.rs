use std::fmt;

use crate::StoichiometryValidationError;

/// A validated limiting-reagent label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LimitingReagent(String);

impl LimitingReagent {
    /// Creates a limiting-reagent label.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::EmptyLimitingReagentLabel`] when `label`
    /// is empty or whitespace only.
    pub fn new(label: &str) -> Result<Self, StoichiometryValidationError> {
        let label = label.trim();

        if label.is_empty() {
            Err(StoichiometryValidationError::EmptyLimitingReagentLabel)
        } else {
            Ok(Self(label.to_owned()))
        }
    }

    /// Returns the label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the label and returns the owned text.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for LimitingReagent {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for LimitingReagent {
    type Error = StoichiometryValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for LimitingReagent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
