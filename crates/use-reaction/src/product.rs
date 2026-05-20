use std::fmt;

use use_chemical_formula::ChemicalFormula;
use use_stoichiometry::StoichiometricCoefficient;

use crate::ReactionTerm;

/// A product-side reaction term.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product(ReactionTerm);

impl Product {
    /// Creates a product wrapper.
    #[must_use]
    pub const fn new(term: ReactionTerm) -> Self {
        Self(term)
    }

    /// Returns the wrapped reaction term.
    #[must_use]
    pub const fn as_term(&self) -> &ReactionTerm {
        &self.0
    }

    /// Consumes this value and returns the wrapped reaction term.
    #[must_use]
    pub fn into_term(self) -> ReactionTerm {
        self.0
    }

    /// Returns the stoichiometric coefficient.
    #[must_use]
    pub const fn coefficient(&self) -> StoichiometricCoefficient {
        self.0.coefficient()
    }

    /// Returns the chemical formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        self.0.formula()
    }
}

impl From<ReactionTerm> for Product {
    fn from(term: ReactionTerm) -> Self {
        Self::new(term)
    }
}

impl fmt::Display for Product {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
