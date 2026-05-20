use std::error::Error;
use std::fmt;

/// Errors returned when constructing compound values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompoundValidationError {
    /// A primary compound name is empty.
    EmptyName,
    /// A common name is empty.
    EmptyCommonName,
    /// A systematic name is empty.
    EmptySystematicName,
    /// An identifier registry namespace is empty.
    EmptyIdentifierNamespace,
    /// An identifier value is empty.
    EmptyIdentifierValue,
}

impl fmt::Display for CompoundValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("compound name must not be empty"),
            Self::EmptyCommonName => formatter.write_str("common name must not be empty"),
            Self::EmptySystematicName => formatter.write_str("systematic name must not be empty"),
            Self::EmptyIdentifierNamespace => {
                formatter.write_str("compound identifier namespace must not be empty")
            },
            Self::EmptyIdentifierValue => {
                formatter.write_str("compound identifier value must not be empty")
            },
        }
    }
}

impl Error for CompoundValidationError {}
