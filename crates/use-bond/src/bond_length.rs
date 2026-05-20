use std::fmt;

use crate::BondValidationError;

/// A positive displayable bond length.
#[derive(Clone, Debug, PartialEq)]
pub struct BondLength {
    value: f64,
    unit: String,
}

impl BondLength {
    /// Creates a bond length.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::NonFiniteBondLength`] when `value` is not finite,
    /// [`BondValidationError::NonPositiveBondLength`] when `value` is zero or negative, or
    /// [`BondValidationError::EmptyLengthUnit`] when `unit` is empty after trimming.
    pub fn new(value: f64, unit: &str) -> Result<Self, BondValidationError> {
        if !value.is_finite() {
            return Err(BondValidationError::NonFiniteBondLength);
        }
        if value <= 0.0 {
            return Err(BondValidationError::NonPositiveBondLength);
        }

        let trimmed = unit.trim();
        if trimmed.is_empty() {
            Err(BondValidationError::EmptyLengthUnit)
        } else {
            Ok(Self {
                value,
                unit: trimmed.to_owned(),
            })
        }
    }

    /// Returns the bond length value.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }

    /// Returns the bond length unit label.
    #[must_use]
    pub fn unit(&self) -> &str {
        &self.unit
    }
}

impl fmt::Display for BondLength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.value, self.unit)
    }
}
