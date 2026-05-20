use std::error::Error;
use std::fmt;

use use_stoichiometry::StoichiometryValidationError;

/// Errors returned when constructing or validating reaction values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReactionValidationError {
    /// A reaction has no reactants and no products.
    EmptyReaction,
    /// A reaction has no reactants.
    MissingReactants,
    /// A reaction has no products.
    MissingProducts,
    /// A catalyst label is empty.
    EmptyCatalystLabel,
    /// A solvent label is empty.
    EmptySolventLabel,
    /// A temperature label is empty.
    EmptyTemperatureLabel,
    /// A pressure label is empty.
    EmptyPressureLabel,
    /// A generic condition label is empty.
    EmptyConditionLabel,
    /// A generic condition value is empty.
    EmptyConditionValue,
    /// A stoichiometry primitive rejected the value.
    InvalidStoichiometry(StoichiometryValidationError),
}

impl fmt::Display for ReactionValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyReaction => {
                formatter.write_str("reaction must contain reactants and products")
            },
            Self::MissingReactants => {
                formatter.write_str("reaction must contain at least one reactant")
            },
            Self::MissingProducts => {
                formatter.write_str("reaction must contain at least one product")
            },
            Self::EmptyCatalystLabel => formatter.write_str("catalyst label must not be empty"),
            Self::EmptySolventLabel => formatter.write_str("solvent label must not be empty"),
            Self::EmptyTemperatureLabel => {
                formatter.write_str("temperature label must not be empty")
            },
            Self::EmptyPressureLabel => formatter.write_str("pressure label must not be empty"),
            Self::EmptyConditionLabel => formatter.write_str("condition label must not be empty"),
            Self::EmptyConditionValue => formatter.write_str("condition value must not be empty"),
            Self::InvalidStoichiometry(error) => {
                write!(formatter, "invalid stoichiometry: {error}")
            },
        }
    }
}

impl Error for ReactionValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidStoichiometry(error) => Some(error),
            _ => None,
        }
    }
}

impl From<StoichiometryValidationError> for ReactionValidationError {
    fn from(error: StoichiometryValidationError) -> Self {
        Self::InvalidStoichiometry(error)
    }
}
