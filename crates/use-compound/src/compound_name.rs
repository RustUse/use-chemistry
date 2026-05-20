use std::fmt;

use crate::CompoundValidationError;

/// A validated primary compound name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompoundName(String);

impl CompoundName {
    /// Creates a compound name.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyName`] when `name` is empty after trimming.
    pub fn new(name: &str) -> Result<Self, CompoundValidationError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(CompoundValidationError::EmptyName)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the compound name text.
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

impl AsRef<str> for CompoundName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for CompoundName {
    type Error = CompoundValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for CompoundName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated common compound name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommonName(String);

impl CommonName {
    /// Creates a common name.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptyCommonName`] when `name` is empty after trimming.
    pub fn new(name: &str) -> Result<Self, CompoundValidationError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(CompoundValidationError::EmptyCommonName)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the common name text.
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

impl AsRef<str> for CommonName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for CommonName {
    type Error = CompoundValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for CommonName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A validated systematic compound name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystematicName(String);

impl SystematicName {
    /// Creates a systematic name.
    ///
    /// # Errors
    ///
    /// Returns [`CompoundValidationError::EmptySystematicName`] when `name` is empty after trimming.
    pub fn new(name: &str) -> Result<Self, CompoundValidationError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            Err(CompoundValidationError::EmptySystematicName)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// Returns the systematic name text.
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

impl AsRef<str> for SystematicName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for SystematicName {
    type Error = CompoundValidationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for SystematicName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
