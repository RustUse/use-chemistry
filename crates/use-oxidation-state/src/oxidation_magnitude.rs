use std::fmt;

use crate::OxidationStateValidationError;

/// A bounded oxidation-state magnitude.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OxidationMagnitude(u8);

impl OxidationMagnitude {
    /// The maximum magnitude supported by this crate.
    pub const MAX: u8 = 8;

    /// Zero magnitude.
    pub const ZERO: Self = Self(0);

    /// Creates an oxidation-state magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::MagnitudeAboveMaximum`] when `magnitude`
    /// is greater than [`Self::MAX`].
    pub const fn new(magnitude: u8) -> Result<Self, OxidationStateValidationError> {
        if magnitude > Self::MAX {
            Err(OxidationStateValidationError::MagnitudeAboveMaximum {
                magnitude,
                maximum: Self::MAX,
            })
        } else {
            Ok(Self(magnitude))
        }
    }

    /// Returns the magnitude value.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }

    /// Returns `true` when the magnitude is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl Default for OxidationMagnitude {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for OxidationMagnitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
