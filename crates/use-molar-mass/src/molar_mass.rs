use std::fmt;

use crate::{MolarMassUnit, MolarMassValidationError};

/// A positive finite molar mass value with an associated display unit.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MolarMass {
    value: f64,
    unit: MolarMassUnit,
}

impl MolarMass {
    /// Creates a molar mass value in the requested unit.
    ///
    /// # Errors
    ///
    /// Returns [`MolarMassValidationError::NonFiniteMolarMass`] when `value` is
    /// not finite, or [`MolarMassValidationError::NonPositiveMolarMass`] when it
    /// is zero or negative.
    pub fn new(value: f64, unit: MolarMassUnit) -> Result<Self, MolarMassValidationError> {
        validate_molar_mass(value)?;

        Ok(Self { value, unit })
    }

    /// Creates a molar mass in grams per mole.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error when `value` is not finite and positive.
    pub fn grams_per_mole(value: f64) -> Result<Self, MolarMassValidationError> {
        Self::new(value, MolarMassUnit::GramsPerMole)
    }

    /// Creates a molar mass in kilograms per mole.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error when `value` is not finite and positive.
    pub fn kilograms_per_mole(value: f64) -> Result<Self, MolarMassValidationError> {
        Self::new(value, MolarMassUnit::KilogramsPerMole)
    }

    /// Returns the numeric molar mass value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns the molar mass display unit.
    #[must_use]
    pub const fn unit(self) -> MolarMassUnit {
        self.unit
    }
}

impl fmt::Display for MolarMass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.value, self.unit)
    }
}

fn validate_molar_mass(value: f64) -> Result<(), MolarMassValidationError> {
    if !value.is_finite() {
        Err(MolarMassValidationError::NonFiniteMolarMass)
    } else if value <= 0.0 {
        Err(MolarMassValidationError::NonPositiveMolarMass)
    } else {
        Ok(())
    }
}
