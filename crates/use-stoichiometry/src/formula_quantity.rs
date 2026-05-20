use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{StoichiometricCoefficient, StoichiometricTerm, StoichiometryValidationError};

/// A formula with a stoichiometric quantity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaQuantity {
    term: StoichiometricTerm,
}

impl FormulaQuantity {
    /// Creates a formula quantity.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] if the coefficient is
    /// structurally invalid.
    pub fn new(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self {
            term: StoichiometricTerm::new(coefficient, formula)?,
        })
    }

    /// Creates a formula quantity from a raw coefficient value.
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
        self.term.coefficient()
    }

    /// Returns the formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        self.term.formula()
    }

    /// Returns the stoichiometric term.
    #[must_use]
    pub const fn term(&self) -> &StoichiometricTerm {
        &self.term
    }

    /// Consumes the quantity and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (StoichiometricCoefficient, ChemicalFormula) {
        self.term.into_parts()
    }
}

impl fmt::Display for FormulaQuantity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.term)
    }
}
