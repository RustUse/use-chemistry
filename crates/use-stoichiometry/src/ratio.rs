use std::fmt;

use crate::{StoichiometricCoefficient, StoichiometryValidationError};

/// A ratio between two stoichiometric coefficients.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StoichiometricRatio {
    numerator: StoichiometricCoefficient,
    denominator: StoichiometricCoefficient,
}

impl StoichiometricRatio {
    /// Creates a stoichiometric ratio from validated coefficients.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroRatioDenominator`] if the denominator
    /// is structurally invalid.
    pub const fn new(
        numerator: StoichiometricCoefficient,
        denominator: StoichiometricCoefficient,
    ) -> Result<Self, StoichiometryValidationError> {
        if denominator.value() == 0 {
            Err(StoichiometryValidationError::ZeroRatioDenominator)
        } else {
            Ok(Self {
                numerator,
                denominator,
            })
        }
    }

    /// Creates a stoichiometric ratio from raw coefficient values.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroRatioDenominator`] when `denominator`
    /// is zero, or [`StoichiometryValidationError::ZeroCoefficient`] when `numerator` is
    /// zero.
    pub const fn from_values(
        numerator: u32,
        denominator: u32,
    ) -> Result<Self, StoichiometryValidationError> {
        if denominator == 0 {
            return Err(StoichiometryValidationError::ZeroRatioDenominator);
        }

        let numerator = match StoichiometricCoefficient::new(numerator) {
            Ok(numerator) => numerator,
            Err(error) => return Err(error),
        };
        let denominator = match StoichiometricCoefficient::new(denominator) {
            Ok(denominator) => denominator,
            Err(error) => return Err(error),
        };

        Self::new(numerator, denominator)
    }

    /// Returns the numerator coefficient.
    #[must_use]
    pub const fn numerator(self) -> StoichiometricCoefficient {
        self.numerator
    }

    /// Returns the denominator coefficient.
    #[must_use]
    pub const fn denominator(self) -> StoichiometricCoefficient {
        self.denominator
    }
}

impl fmt::Display for StoichiometricRatio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.numerator, self.denominator)
    }
}
