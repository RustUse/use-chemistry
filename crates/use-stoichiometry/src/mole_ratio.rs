use std::fmt;

use crate::{StoichiometricCoefficient, StoichiometricRatio, StoichiometryValidationError};

/// A mole ratio between two stoichiometric coefficients.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoleRatio(StoichiometricRatio);

impl MoleRatio {
    /// Creates a mole ratio from validated coefficients.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroRatioDenominator`] if the denominator
    /// is structurally invalid.
    pub fn new(
        numerator: StoichiometricCoefficient,
        denominator: StoichiometricCoefficient,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self(StoichiometricRatio::new(numerator, denominator)?))
    }

    /// Creates a mole ratio from raw coefficient values.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroRatioDenominator`] when `denominator`
    /// is zero, or [`StoichiometryValidationError::ZeroCoefficient`] when `numerator` is
    /// zero.
    pub fn from_values(
        numerator: u32,
        denominator: u32,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self(StoichiometricRatio::from_values(
            numerator,
            denominator,
        )?))
    }

    /// Returns the wrapped stoichiometric ratio.
    #[must_use]
    pub const fn as_ratio(self) -> StoichiometricRatio {
        self.0
    }

    /// Returns the numerator coefficient.
    #[must_use]
    pub const fn numerator(self) -> StoichiometricCoefficient {
        self.0.numerator()
    }

    /// Returns the denominator coefficient.
    #[must_use]
    pub const fn denominator(self) -> StoichiometricCoefficient {
        self.0.denominator()
    }
}

impl fmt::Display for MoleRatio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
