use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{StoichiometricCoefficient, StoichiometryValidationError};

/// A stoichiometric coefficient paired with a formula.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoichiometricTerm {
    coefficient: StoichiometricCoefficient,
    formula: ChemicalFormula,
}

impl StoichiometricTerm {
    /// Creates a stoichiometric term from validated parts.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] if the coefficient is
    /// structurally invalid.
    pub fn new(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
    ) -> Result<Self, StoichiometryValidationError> {
        if coefficient.value() == 0 {
            Err(StoichiometryValidationError::ZeroCoefficient)
        } else {
            Ok(Self {
                coefficient,
                formula,
            })
        }
    }

    /// Creates a stoichiometric term from a raw coefficient value.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] when `coefficient` is zero.
    pub fn from_value(
        coefficient: u32,
        formula: ChemicalFormula,
    ) -> Result<Self, StoichiometryValidationError> {
        Self::new(StoichiometricCoefficient::new(coefficient)?, formula)
    }

    /// Returns the coefficient.
    #[must_use]
    pub const fn coefficient(&self) -> StoichiometricCoefficient {
        self.coefficient
    }

    /// Returns the formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        &self.formula
    }

    /// Consumes the term and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (StoichiometricCoefficient, ChemicalFormula) {
        (self.coefficient, self.formula)
    }
}

impl fmt::Display for StoichiometricTerm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficient.is_one() {
            write!(formatter, "{}", self.formula)
        } else {
            write!(formatter, "{}{}", self.coefficient, self.formula)
        }
    }
}
