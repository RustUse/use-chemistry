use std::fmt;

use crate::ReactionValidationError;

/// A lightweight catalyst descriptor.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Catalyst(String);

impl Catalyst {
    /// Creates a catalyst descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::EmptyCatalystLabel`] when `label` is empty after
    /// trimming.
    pub fn new(label: &str) -> Result<Self, ReactionValidationError> {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            Err(ReactionValidationError::EmptyCatalystLabel)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the catalyst descriptor text.
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

impl AsRef<str> for Catalyst {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for Catalyst {
    type Error = ReactionValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for Catalyst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
