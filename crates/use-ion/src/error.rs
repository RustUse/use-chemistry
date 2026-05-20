use std::error::Error;
use std::fmt;

/// Errors returned when constructing ion values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IonValidationError {
    /// An ion name is empty.
    EmptyName,
    /// A charge magnitude is zero.
    ZeroChargeMagnitude,
    /// An oxidation-state label is empty.
    EmptyOxidationStateLabel,
    /// A positive ion was required.
    ExpectedCation,
    /// A negative ion was required.
    ExpectedAnion,
    /// A monatomic formula was required.
    ExpectedMonatomicFormula,
    /// A polyatomic formula was required.
    ExpectedPolyatomicFormula,
}

impl fmt::Display for IonValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("ion name must not be empty"),
            Self::ZeroChargeMagnitude => {
                formatter.write_str("ion charge magnitude must be greater than zero")
            },
            Self::EmptyOxidationStateLabel => {
                formatter.write_str("oxidation-state label must not be empty")
            },
            Self::ExpectedCation => formatter.write_str("ion must have a positive charge"),
            Self::ExpectedAnion => formatter.write_str("ion must have a negative charge"),
            Self::ExpectedMonatomicFormula => {
                formatter.write_str("ion formula must contain exactly one atom")
            },
            Self::ExpectedPolyatomicFormula => {
                formatter.write_str("ion formula must contain more than one atom")
            },
        }
    }
}

impl Error for IonValidationError {}
