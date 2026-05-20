use std::fmt;

use crate::StoichiometryValidationError;

/// A positive finite theoretical yield value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TheoreticalYield(f64);

impl TheoreticalYield {
    /// Creates a theoretical yield value.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::NonFiniteYield`] when `value` is not
    /// finite, or [`StoichiometryValidationError::NonPositiveTheoreticalYield`] when it
    /// is zero or negative.
    pub fn new(value: f64) -> Result<Self, StoichiometryValidationError> {
        validate_finite_yield(value)?;

        if value <= 0.0 {
            Err(StoichiometryValidationError::NonPositiveTheoreticalYield)
        } else {
            Ok(Self(value))
        }
    }

    /// Returns the yield value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for TheoreticalYield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A nonnegative finite actual yield value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ActualYield(f64);

impl ActualYield {
    /// Creates an actual yield value.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::NonFiniteYield`] when `value` is not
    /// finite, or [`StoichiometryValidationError::NegativeYield`] when it is negative.
    pub fn new(value: f64) -> Result<Self, StoichiometryValidationError> {
        validate_nonnegative_yield(value).map(Self)
    }

    /// Returns the yield value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for ActualYield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A nonnegative finite percent yield value.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PercentYield(f64);

impl PercentYield {
    /// Creates a percent yield value.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::NonFiniteYield`] when `value` is not
    /// finite, or [`StoichiometryValidationError::NegativeYield`] when it is negative.
    pub fn new(value: f64) -> Result<Self, StoichiometryValidationError> {
        validate_nonnegative_yield(value).map(Self)
    }

    /// Calculates percent yield from actual and theoretical yield values.
    ///
    /// # Errors
    ///
    /// Returns a validation error when either yield value is invalid, or when the
    /// theoretical yield is zero or negative.
    pub fn from_actual_and_theoretical(
        actual: f64,
        theoretical: f64,
    ) -> Result<Self, StoichiometryValidationError> {
        Self::from_yields(
            ActualYield::new(actual)?,
            TheoreticalYield::new(theoretical)?,
        )
    }

    /// Calculates percent yield from validated yield wrappers.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::NonFiniteYield`] if the calculated percent
    /// yield is not finite.
    pub fn from_yields(
        actual: ActualYield,
        theoretical: TheoreticalYield,
    ) -> Result<Self, StoichiometryValidationError> {
        Self::new((actual.value() / theoretical.value()) * 100.0)
    }

    /// Returns the percent yield value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for PercentYield {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}%", self.0)
    }
}

fn validate_finite_yield(value: f64) -> Result<(), StoichiometryValidationError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(StoichiometryValidationError::NonFiniteYield)
    }
}

fn validate_nonnegative_yield(value: f64) -> Result<f64, StoichiometryValidationError> {
    validate_finite_yield(value)?;

    if value < 0.0 {
        Err(StoichiometryValidationError::NegativeYield)
    } else {
        Ok(value)
    }
}
