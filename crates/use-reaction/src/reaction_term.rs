use std::fmt;

use use_chemical_formula::ChemicalFormula;
use use_stoichiometry::{StoichiometricCoefficient, StoichiometricTerm};

use crate::ReactionValidationError;

/// A formula-backed reaction term with a stoichiometric coefficient.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactionTerm {
    term: StoichiometricTerm,
}

impl ReactionTerm {
    /// Creates a reaction term with coefficient one.
    #[must_use]
    pub fn new(formula: ChemicalFormula) -> Self {
        match Self::from_value(1, formula) {
            Ok(term) => term,
            Err(error) => unreachable!("coefficient one should be valid: {error}"),
        }
    }

    /// Creates a reaction term from validated parts.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::InvalidStoichiometry`] if the stoichiometry primitive
    /// rejects the coefficient.
    pub fn from_parts(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
    ) -> Result<Self, ReactionValidationError> {
        Ok(Self {
            term: StoichiometricTerm::new(coefficient, formula)?,
        })
    }

    /// Creates a reaction term from a raw coefficient value.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::InvalidStoichiometry`] when `coefficient` is zero.
    pub fn from_value(
        coefficient: u32,
        formula: ChemicalFormula,
    ) -> Result<Self, ReactionValidationError> {
        Self::from_parts(StoichiometricCoefficient::new(coefficient)?, formula)
    }

    /// Returns a copy of this term with a new coefficient.
    ///
    /// # Errors
    ///
    /// Returns [`ReactionValidationError::InvalidStoichiometry`] when `coefficient` is zero.
    pub fn with_coefficient(self, coefficient: u32) -> Result<Self, ReactionValidationError> {
        let (_, formula) = self.term.into_parts();
        Self::from_value(coefficient, formula)
    }

    /// Returns the stoichiometric coefficient.
    #[must_use]
    pub const fn coefficient(&self) -> StoichiometricCoefficient {
        self.term.coefficient()
    }

    /// Returns the chemical formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        self.term.formula()
    }

    /// Returns the wrapped stoichiometric term.
    #[must_use]
    pub const fn as_stoichiometric_term(&self) -> &StoichiometricTerm {
        &self.term
    }

    /// Consumes this value and returns the wrapped stoichiometric term.
    #[must_use]
    pub fn into_stoichiometric_term(self) -> StoichiometricTerm {
        self.term
    }

    /// Consumes the term and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (StoichiometricCoefficient, ChemicalFormula) {
        self.term.into_parts()
    }
}

impl From<StoichiometricTerm> for ReactionTerm {
    fn from(term: StoichiometricTerm) -> Self {
        Self { term }
    }
}

impl fmt::Display for ReactionTerm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.term)
    }
}
