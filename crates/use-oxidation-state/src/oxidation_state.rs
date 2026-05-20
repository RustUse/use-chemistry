use std::fmt;

use crate::{
    OxidationMagnitude, OxidationSign, OxidationStateValidationError, RomanOxidationState,
};

/// A validated oxidation-state value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OxidationState {
    sign: OxidationSign,
    magnitude: OxidationMagnitude,
}

impl OxidationState {
    /// Creates an oxidation state from a sign and magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::ZeroSignedMagnitude`] when a positive or
    /// negative state uses zero magnitude, or
    /// [`OxidationStateValidationError::NonZeroZeroMagnitude`] when a zero state uses a
    /// nonzero magnitude.
    pub const fn new(
        sign: OxidationSign,
        magnitude: OxidationMagnitude,
    ) -> Result<Self, OxidationStateValidationError> {
        match (sign, magnitude.get()) {
            (OxidationSign::Positive | OxidationSign::Negative, 0) => {
                Err(OxidationStateValidationError::ZeroSignedMagnitude)
            },
            (OxidationSign::Zero, 0) => Ok(Self { sign, magnitude }),
            (OxidationSign::Zero, _) => Err(OxidationStateValidationError::NonZeroZeroMagnitude),
            _ => Ok(Self { sign, magnitude }),
        }
    }

    /// Creates a positive oxidation state.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::ZeroSignedMagnitude`] when `magnitude` is
    /// zero, or [`OxidationStateValidationError::MagnitudeAboveMaximum`] when it is above
    /// the supported range.
    pub fn positive(magnitude: u8) -> Result<Self, OxidationStateValidationError> {
        let magnitude = OxidationMagnitude::new(magnitude)?;

        Self::new(OxidationSign::Positive, magnitude)
    }

    /// Creates a negative oxidation state.
    ///
    /// # Errors
    ///
    /// Returns [`OxidationStateValidationError::ZeroSignedMagnitude`] when `magnitude` is
    /// zero, or [`OxidationStateValidationError::MagnitudeAboveMaximum`] when it is above
    /// the supported range.
    pub fn negative(magnitude: u8) -> Result<Self, OxidationStateValidationError> {
        let magnitude = OxidationMagnitude::new(magnitude)?;

        Self::new(OxidationSign::Negative, magnitude)
    }

    /// Creates a zero oxidation state.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            sign: OxidationSign::Zero,
            magnitude: OxidationMagnitude::ZERO,
        }
    }

    /// Returns the oxidation-state sign.
    #[must_use]
    pub const fn sign(self) -> OxidationSign {
        self.sign
    }

    /// Returns the oxidation-state magnitude.
    #[must_use]
    pub const fn magnitude(self) -> OxidationMagnitude {
        self.magnitude
    }

    /// Returns the oxidation-state magnitude value.
    #[must_use]
    pub const fn magnitude_value(self) -> u8 {
        self.magnitude.get()
    }

    /// Returns the signed oxidation-state value.
    #[must_use]
    pub const fn signed_value(self) -> i8 {
        match self.sign {
            OxidationSign::Positive => self.magnitude.get() as i8,
            OxidationSign::Negative => -(self.magnitude.get() as i8),
            OxidationSign::Zero => 0,
        }
    }

    /// Returns `true` for positive oxidation states.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.sign.is_positive()
    }

    /// Returns `true` for negative oxidation states.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.sign.is_negative()
    }

    /// Returns `true` for zero oxidation states.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.sign.is_zero()
    }

    /// Returns the Roman numeral for the nonzero magnitude.
    #[must_use]
    pub fn to_roman(self) -> Option<String> {
        RomanOxidationState::from_state(self).map(|roman| roman.to_string())
    }
}

impl Default for OxidationState {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Display for OxidationState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.sign {
            OxidationSign::Positive => write!(formatter, "+{}", self.magnitude),
            OxidationSign::Negative => write!(formatter, "-{}", self.magnitude),
            OxidationSign::Zero => formatter.write_str("0"),
        }
    }
}
