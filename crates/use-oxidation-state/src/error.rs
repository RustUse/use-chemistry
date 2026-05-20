use std::error::Error;
use std::fmt;

/// Errors returned when constructing oxidation-state values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OxidationStateValidationError {
    /// A signed oxidation state used zero magnitude.
    ZeroSignedMagnitude,
    /// A zero oxidation state used nonzero magnitude.
    NonZeroZeroMagnitude,
    /// A magnitude is above the supported range.
    MagnitudeAboveMaximum { magnitude: u8, maximum: u8 },
    /// A generic assignment label is empty.
    EmptyAssignmentLabel,
    /// An element symbol is empty.
    EmptyElementSymbol,
    /// An element symbol does not match the supported shape.
    InvalidElementSymbol(String),
    /// A formula or formula-context label is empty.
    EmptyFormulaLabel,
}

impl fmt::Display for OxidationStateValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSignedMagnitude => {
                formatter.write_str("signed oxidation-state magnitude must be greater than zero")
            },
            Self::NonZeroZeroMagnitude => {
                formatter.write_str("zero oxidation state must have zero magnitude")
            },
            Self::MagnitudeAboveMaximum { magnitude, maximum } => write!(
                formatter,
                "oxidation-state magnitude {magnitude} is greater than maximum {maximum}"
            ),
            Self::EmptyAssignmentLabel => {
                formatter.write_str("oxidation-state assignment label must not be empty")
            },
            Self::EmptyElementSymbol => formatter.write_str("element symbol must not be empty"),
            Self::InvalidElementSymbol(symbol) => {
                write!(formatter, "invalid element symbol: {symbol}")
            },
            Self::EmptyFormulaLabel => {
                formatter.write_str("formula oxidation-state label must not be empty")
            },
        }
    }
}

impl Error for OxidationStateValidationError {}
