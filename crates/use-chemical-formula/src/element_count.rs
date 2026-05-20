use std::fmt;

use crate::FormulaValidationError;

/// A positive element count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ElementCount(u32);

impl ElementCount {
    /// The implicit count for a term with no numeric suffix.
    pub const ONE: Self = Self(1);

    /// Creates a positive element count.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaValidationError::ZeroCount`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, FormulaValidationError> {
        if value == 0 {
            Err(FormulaValidationError::ZeroCount)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns `true` when the count is one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1
    }
}

impl Default for ElementCount {
    fn default() -> Self {
        Self::ONE
    }
}

impl fmt::Display for ElementCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A positive group or hydrate multiplier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaMultiplier(u32);

impl FormulaMultiplier {
    /// The implicit multiplier for a group or hydrate with no numeric suffix.
    pub const ONE: Self = Self(1);

    /// Creates a positive formula multiplier.
    ///
    /// # Errors
    ///
    /// Returns [`FormulaValidationError::ZeroMultiplier`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, FormulaValidationError> {
        if value == 0 {
            Err(FormulaValidationError::ZeroMultiplier)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the numeric multiplier.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    /// Returns `true` when the multiplier is one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1
    }
}

impl Default for FormulaMultiplier {
    fn default() -> Self {
        Self::ONE
    }
}

impl fmt::Display for FormulaMultiplier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
