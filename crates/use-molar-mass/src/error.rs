use std::error::Error;
use std::fmt;

/// Errors returned while constructing or calculating molar mass values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MolarMassValidationError {
    /// A molar mass value was not finite.
    NonFiniteMolarMass,
    /// A molar mass value was zero or negative.
    NonPositiveMolarMass,
    /// An element symbol does not match the supported chemical-formula shape.
    InvalidElementSymbol(String),
    /// An atomic mass value was not finite.
    NonFiniteAtomicMass { symbol: String },
    /// An atomic mass value was zero or negative.
    NonPositiveAtomicMass { symbol: String },
    /// An expanded formula count was zero.
    ZeroElementCount { symbol: String },
    /// An expanded formula count cannot fit the contribution count type.
    FormulaCountTooLarge { symbol: String, count: u64 },
    /// No atomic mass entry exists for a required element symbol.
    MissingAtomicMass { symbol: String },
}

impl fmt::Display for MolarMassValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteMolarMass => formatter.write_str("molar mass must be finite"),
            Self::NonPositiveMolarMass => {
                formatter.write_str("molar mass must be greater than zero")
            },
            Self::InvalidElementSymbol(symbol) => {
                write!(formatter, "invalid element symbol: {symbol}")
            },
            Self::NonFiniteAtomicMass { symbol } => {
                write!(formatter, "atomic mass for {symbol} must be finite")
            },
            Self::NonPositiveAtomicMass { symbol } => {
                write!(
                    formatter,
                    "atomic mass for {symbol} must be greater than zero"
                )
            },
            Self::ZeroElementCount { symbol } => {
                write!(
                    formatter,
                    "element count for {symbol} must be greater than zero"
                )
            },
            Self::FormulaCountTooLarge { symbol, count } => {
                write!(
                    formatter,
                    "element count for {symbol} is too large: {count}"
                )
            },
            Self::MissingAtomicMass { symbol } => {
                write!(
                    formatter,
                    "missing atomic mass for element symbol: {symbol}"
                )
            },
        }
    }
}

impl Error for MolarMassValidationError {}
