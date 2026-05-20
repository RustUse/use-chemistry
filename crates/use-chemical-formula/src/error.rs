use std::error::Error;
use std::fmt;

/// Errors returned while parsing a formula string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaParseError {
    /// The input is empty or whitespace only.
    EmptyFormula,
    /// A formula part has no terms.
    EmptyPart,
    /// A parenthesized group has no terms.
    EmptyGroup,
    /// An element symbol does not match the supported shape.
    InvalidSymbol(String),
    /// A numeric count or multiplier could not be represented.
    InvalidNumber(String),
    /// An element count was zero.
    ZeroCount,
    /// A group or hydrate multiplier was zero.
    ZeroMultiplier,
    /// The parser encountered an unexpected character.
    UnexpectedCharacter(char),
    /// The input ended while a construct was still open.
    UnexpectedEnd,
    /// A group was opened but not closed.
    UnmatchedOpenGroup,
    /// A closing parenthesis appeared without a matching opening parenthesis.
    UnmatchedCloseGroup,
    /// A hydrate separator appeared at the end of the formula.
    TrailingSeparator,
}

impl fmt::Display for FormulaParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFormula => formatter.write_str("formula is empty"),
            Self::EmptyPart => formatter.write_str("formula part is empty"),
            Self::EmptyGroup => formatter.write_str("formula group is empty"),
            Self::InvalidSymbol(symbol) => write!(formatter, "invalid element symbol: {symbol}"),
            Self::InvalidNumber(number) => write!(formatter, "invalid formula number: {number}"),
            Self::ZeroCount => formatter.write_str("element count must be greater than zero"),
            Self::ZeroMultiplier => {
                formatter.write_str("formula multiplier must be greater than zero")
            },
            Self::UnexpectedCharacter(character) => {
                write!(formatter, "unexpected character in formula: {character}")
            },
            Self::UnexpectedEnd => formatter.write_str("unexpected end of formula"),
            Self::UnmatchedOpenGroup => {
                formatter.write_str("formula group is missing a closing parenthesis")
            },
            Self::UnmatchedCloseGroup => {
                formatter.write_str("formula has an unmatched closing parenthesis")
            },
            Self::TrailingSeparator => {
                formatter.write_str("hydrate separator must be followed by a formula part")
            },
        }
    }
}

impl Error for FormulaParseError {}

/// Errors returned when constructing formula values directly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaValidationError {
    /// A term list is empty.
    EmptyTerms,
    /// A formula part has no terms.
    EmptyPart,
    /// A group has no terms.
    EmptyGroup,
    /// An element symbol does not match the supported shape.
    InvalidSymbol(String),
    /// An element count was zero.
    ZeroCount,
    /// A group or hydrate multiplier was zero.
    ZeroMultiplier,
}

impl fmt::Display for FormulaValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTerms => formatter.write_str("formula term list is empty"),
            Self::EmptyPart => formatter.write_str("formula part is empty"),
            Self::EmptyGroup => formatter.write_str("formula group is empty"),
            Self::InvalidSymbol(symbol) => write!(formatter, "invalid element symbol: {symbol}"),
            Self::ZeroCount => formatter.write_str("element count must be greater than zero"),
            Self::ZeroMultiplier => {
                formatter.write_str("formula multiplier must be greater than zero")
            },
        }
    }
}

impl Error for FormulaValidationError {}

impl From<FormulaValidationError> for FormulaParseError {
    fn from(error: FormulaValidationError) -> Self {
        match error {
            FormulaValidationError::EmptyTerms | FormulaValidationError::EmptyPart => {
                Self::EmptyPart
            },
            FormulaValidationError::EmptyGroup => Self::EmptyGroup,
            FormulaValidationError::InvalidSymbol(symbol) => Self::InvalidSymbol(symbol),
            FormulaValidationError::ZeroCount => Self::ZeroCount,
            FormulaValidationError::ZeroMultiplier => Self::ZeroMultiplier,
        }
    }
}
