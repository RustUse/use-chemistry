use std::fmt;

use crate::{ChargeSign, IonValidationError};

/// A nonzero ionic charge magnitude.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChargeMagnitude(u8);

impl ChargeMagnitude {
    /// Creates a charge magnitude.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ZeroChargeMagnitude`] when `magnitude` is zero.
    pub const fn new(magnitude: u8) -> Result<Self, IonValidationError> {
        if magnitude == 0 {
            Err(IonValidationError::ZeroChargeMagnitude)
        } else {
            Ok(Self(magnitude))
        }
    }

    /// Returns the magnitude value.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for ChargeMagnitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A nonzero signed ionic charge.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IonCharge {
    sign: ChargeSign,
    magnitude: ChargeMagnitude,
}

impl IonCharge {
    /// Creates a positive ion charge.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ZeroChargeMagnitude`] when `magnitude` is zero.
    pub const fn positive(magnitude: u8) -> Result<Self, IonValidationError> {
        Ok(Self {
            sign: ChargeSign::Positive,
            magnitude: match ChargeMagnitude::new(magnitude) {
                Ok(value) => value,
                Err(error) => return Err(error),
            },
        })
    }

    /// Creates a negative ion charge.
    ///
    /// # Errors
    ///
    /// Returns [`IonValidationError::ZeroChargeMagnitude`] when `magnitude` is zero.
    pub const fn negative(magnitude: u8) -> Result<Self, IonValidationError> {
        Ok(Self {
            sign: ChargeSign::Negative,
            magnitude: match ChargeMagnitude::new(magnitude) {
                Ok(value) => value,
                Err(error) => return Err(error),
            },
        })
    }

    /// Returns the charge sign.
    #[must_use]
    pub const fn sign(self) -> ChargeSign {
        self.sign
    }

    /// Returns the charge magnitude as a primitive value.
    #[must_use]
    pub const fn magnitude(self) -> u8 {
        self.magnitude.get()
    }

    /// Returns the charge magnitude wrapper.
    #[must_use]
    pub const fn magnitude_value(self) -> ChargeMagnitude {
        self.magnitude
    }

    /// Returns `true` for a positive charge.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.sign.is_positive()
    }

    /// Returns `true` for a negative charge.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.sign.is_negative()
    }

    /// Returns `true` for a cation charge.
    #[must_use]
    pub const fn is_cation(self) -> bool {
        self.is_positive()
    }

    /// Returns `true` for an anion charge.
    #[must_use]
    pub const fn is_anion(self) -> bool {
        self.is_negative()
    }
}

impl fmt::Display for IonCharge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.magnitude.get() == 1 {
            write!(formatter, "{}", self.sign)
        } else {
            write!(formatter, "{}{}", self.magnitude, self.sign)
        }
    }
}
