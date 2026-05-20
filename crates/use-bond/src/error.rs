use std::error::Error;
use std::fmt;

/// Errors returned when constructing bond values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BondValidationError {
    /// A bond endpoint label is empty.
    EmptyEndpointLabel,
    /// A bond participant label is empty.
    EmptyParticipantLabel,
    /// A bond descriptor is empty.
    EmptyDescriptor,
    /// A bond angle label or reference is empty.
    EmptyAngleLabel,
    /// A bond length unit is empty.
    EmptyLengthUnit,
    /// A bond length value is not finite.
    NonFiniteBondLength,
    /// A bond length value is zero or negative.
    NonPositiveBondLength,
    /// A fractional bond order numerator is zero.
    ZeroFractionalBondOrderNumerator,
    /// A fractional bond order denominator is zero.
    ZeroFractionalBondOrderDenominator,
}

impl fmt::Display for BondValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEndpointLabel => {
                formatter.write_str("bond endpoint label must not be empty")
            },
            Self::EmptyParticipantLabel => {
                formatter.write_str("bond participant label must not be empty")
            },
            Self::EmptyDescriptor => formatter.write_str("bond descriptor must not be empty"),
            Self::EmptyAngleLabel => formatter.write_str("bond angle label must not be empty"),
            Self::EmptyLengthUnit => formatter.write_str("bond length unit must not be empty"),
            Self::NonFiniteBondLength => formatter.write_str("bond length must be finite"),
            Self::NonPositiveBondLength => formatter.write_str("bond length must be positive"),
            Self::ZeroFractionalBondOrderNumerator => {
                formatter.write_str("fractional bond order numerator must not be zero")
            },
            Self::ZeroFractionalBondOrderDenominator => {
                formatter.write_str("fractional bond order denominator must not be zero")
            },
        }
    }
}

impl Error for BondValidationError {}
