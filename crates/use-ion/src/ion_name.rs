use std::fmt;

use crate::IonValidationError;

/// A validated ion name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IonName(String);

impl IonName {
    /// Creates an ion name.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn new(name: &str) -> Result<Self, IonValidationError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(IonValidationError::EmptyName)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the ion name text.
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

impl AsRef<str> for IonName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for IonName {
    type Error = IonValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for IonName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
