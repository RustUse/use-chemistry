use std::fmt;

use crate::StoichiometryValidationError;

/// A nonzero stoichiometric coefficient.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoichiometricCoefficient(u32);

impl StoichiometricCoefficient {
    /// Creates a stoichiometric coefficient.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] when `value` is zero.
    pub const fn new(value: u32) -> Result<Self, StoichiometryValidationError> {
        if value == 0 {
            Err(StoichiometryValidationError::ZeroCoefficient)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the coefficient value.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }

    /// Returns `true` when the coefficient is one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1
    }
}

impl TryFrom<u32> for StoichiometricCoefficient {
    type Error = StoichiometryValidationError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for StoichiometricCoefficient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
