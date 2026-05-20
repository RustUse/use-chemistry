use std::fmt;

use crate::BondValidationError;

/// A rational fractional bond order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FractionalBondOrder {
    numerator: u32,
    denominator: u32,
}

impl FractionalBondOrder {
    /// Creates a fractional bond order.
    ///
    /// # Errors
    ///
    /// Returns [`BondValidationError::ZeroFractionalBondOrderNumerator`] when `numerator` is zero,
    /// or [`BondValidationError::ZeroFractionalBondOrderDenominator`] when `denominator` is zero.
    pub const fn new(numerator: u32, denominator: u32) -> Result<Self, BondValidationError> {
        if numerator == 0 {
            Err(BondValidationError::ZeroFractionalBondOrderNumerator)
        } else if denominator == 0 {
            Err(BondValidationError::ZeroFractionalBondOrderDenominator)
        } else {
            Ok(Self {
                numerator,
                denominator,
            })
        }
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u32 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> u32 {
        self.denominator
    }
}

impl fmt::Display for FractionalBondOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}/{}", self.numerator, self.denominator)
    }
}

/// A lightweight bond order label.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BondOrder {
    /// Single bond order.
    Single,
    /// Double bond order.
    Double,
    /// Triple bond order.
    Triple,
    /// Quadruple bond order.
    Quadruple,
    /// Aromatic bond order.
    Aromatic,
    /// Rational fractional bond order.
    Fractional(FractionalBondOrder),
    /// Unknown or intentionally unspecified bond order.
    Unknown,
}

impl fmt::Display for BondOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single => formatter.write_str("single"),
            Self::Double => formatter.write_str("double"),
            Self::Triple => formatter.write_str("triple"),
            Self::Quadruple => formatter.write_str("quadruple"),
            Self::Aromatic => formatter.write_str("aromatic"),
            Self::Fractional(order) => write!(formatter, "{order}"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}
