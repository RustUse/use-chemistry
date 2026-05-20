use std::error::Error;
use std::fmt;

/// Errors returned when constructing molecule values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MoleculeValidationError {
    /// A molecule name is empty.
    EmptyName,
    /// A builder was finalized without assigning a formula.
    MissingFormula,
    /// An atom label is empty.
    EmptyAtomLabel,
    /// An atom label does not match the supported element-symbol shape.
    InvalidAtomLabel(String),
    /// An atom identifier is empty.
    EmptyAtomId,
    /// A connection order was zero.
    ZeroConnectionOrder,
    /// A connection points from an atom to itself.
    SelfConnection {
        /// The self-connected atom index.
        index: usize,
    },
    /// A connection references an atom index outside the explicit atom list.
    InvalidConnectionIndex {
        /// The invalid atom index.
        index: usize,
        /// The current explicit atom count.
        atom_count: usize,
    },
}

impl fmt::Display for MoleculeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("molecule name must not be empty"),
            Self::MissingFormula => formatter.write_str("molecule formula is required"),
            Self::EmptyAtomLabel => formatter.write_str("atom label must not be empty"),
            Self::InvalidAtomLabel(label) => write!(formatter, "invalid atom label: {label}"),
            Self::EmptyAtomId => formatter.write_str("atom identifier must not be empty"),
            Self::ZeroConnectionOrder => {
                formatter.write_str("atom connection order must not be zero")
            },
            Self::SelfConnection { index } => {
                write!(
                    formatter,
                    "atom connection cannot point index {index} to itself"
                )
            },
            Self::InvalidConnectionIndex { index, atom_count } => write!(
                formatter,
                "atom connection index {index} is outside atom count {atom_count}"
            ),
        }
    }
}

impl Error for MoleculeValidationError {}
