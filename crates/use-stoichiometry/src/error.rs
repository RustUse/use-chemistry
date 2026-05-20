use std::error::Error;
use std::fmt;

/// Errors returned when constructing stoichiometry values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StoichiometryValidationError {
    /// A stoichiometric coefficient is zero.
    ZeroCoefficient,
    /// A ratio denominator is zero.
    ZeroRatioDenominator,
    /// A limiting-reagent label is empty.
    EmptyLimitingReagentLabel,
    /// An excess-reagent label is empty.
    EmptyExcessReagentLabel,
    /// A yield value is not finite.
    NonFiniteYield,
    /// A yield value is negative.
    NegativeYield,
    /// A theoretical yield value is zero or negative.
    NonPositiveTheoreticalYield,
    /// A reactant entry was required.
    ExpectedReactant,
    /// A product entry was required.
    ExpectedProduct,
}

impl fmt::Display for StoichiometryValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroCoefficient => {
                formatter.write_str("stoichiometric coefficient must be greater than zero")
            },
            Self::ZeroRatioDenominator => {
                formatter.write_str("stoichiometric ratio denominator must be greater than zero")
            },
            Self::EmptyLimitingReagentLabel => {
                formatter.write_str("limiting-reagent label must not be empty")
            },
            Self::EmptyExcessReagentLabel => {
                formatter.write_str("excess-reagent label must not be empty")
            },
            Self::NonFiniteYield => formatter.write_str("yield value must be finite"),
            Self::NegativeYield => formatter.write_str("yield value must not be negative"),
            Self::NonPositiveTheoreticalYield => {
                formatter.write_str("theoretical yield must be greater than zero")
            },
            Self::ExpectedReactant => formatter.write_str("reaction entry must be a reactant"),
            Self::ExpectedProduct => formatter.write_str("reaction entry must be a product"),
        }
    }
}

impl Error for StoichiometryValidationError {}
