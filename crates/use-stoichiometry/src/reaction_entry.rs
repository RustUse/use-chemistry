use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{
    ReactionSide, StoichiometricCoefficient, StoichiometricTerm, StoichiometryValidationError,
};

/// A stoichiometric entry on one side of a reaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactionEntry {
    term: StoichiometricTerm,
    side: ReactionSide,
}

impl ReactionEntry {
    /// Creates a reaction entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] if the coefficient is
    /// structurally invalid.
    pub fn new(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
        side: ReactionSide,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self {
            term: StoichiometricTerm::new(coefficient, formula)?,
            side,
        })
    }

    /// Creates a reaction entry from a raw coefficient value.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] when `coefficient` is zero.
    pub fn from_value(
        coefficient: u32,
        formula: ChemicalFormula,
        side: ReactionSide,
    ) -> Result<Self, StoichiometryValidationError> {
        Self::new(StoichiometricCoefficient::new(coefficient)?, formula, side)
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

    /// Returns the reaction side.
    #[must_use]
    pub const fn side(&self) -> ReactionSide {
        self.side
    }

    /// Returns the stoichiometric term.
    #[must_use]
    pub const fn term(&self) -> &StoichiometricTerm {
        &self.term
    }

    /// Consumes the entry and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (StoichiometricCoefficient, ChemicalFormula, ReactionSide) {
        let (coefficient, formula) = self.term.into_parts();
        (coefficient, formula, self.side)
    }
}

impl fmt::Display for ReactionEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.term)
    }
}

/// A reactant-side reaction entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReactantEntry(ReactionEntry);

impl ReactantEntry {
    /// Creates a reactant entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] if the coefficient is
    /// structurally invalid.
    pub fn new(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self(ReactionEntry::new(
            coefficient,
            formula,
            ReactionSide::Reactant,
        )?))
    }

    /// Creates a reactant wrapper from a reaction entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ExpectedReactant`] when the entry is not a
    /// reactant.
    pub fn from_entry(entry: ReactionEntry) -> Result<Self, StoichiometryValidationError> {
        if entry.side().is_reactant() {
            Ok(Self(entry))
        } else {
            Err(StoichiometryValidationError::ExpectedReactant)
        }
    }

    /// Returns the wrapped reaction entry.
    #[must_use]
    pub const fn as_entry(&self) -> &ReactionEntry {
        &self.0
    }

    /// Consumes the wrapper and returns the reaction entry.
    #[must_use]
    pub fn into_entry(self) -> ReactionEntry {
        self.0
    }
}

impl fmt::Display for ReactantEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A product-side reaction entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductEntry(ReactionEntry);

impl ProductEntry {
    /// Creates a product entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ZeroCoefficient`] if the coefficient is
    /// structurally invalid.
    pub fn new(
        coefficient: StoichiometricCoefficient,
        formula: ChemicalFormula,
    ) -> Result<Self, StoichiometryValidationError> {
        Ok(Self(ReactionEntry::new(
            coefficient,
            formula,
            ReactionSide::Product,
        )?))
    }

    /// Creates a product wrapper from a reaction entry.
    ///
    /// # Errors
    ///
    /// Returns [`StoichiometryValidationError::ExpectedProduct`] when the entry is not a
    /// product.
    pub fn from_entry(entry: ReactionEntry) -> Result<Self, StoichiometryValidationError> {
        if entry.side().is_product() {
            Ok(Self(entry))
        } else {
            Err(StoichiometryValidationError::ExpectedProduct)
        }
    }

    /// Returns the wrapped reaction entry.
    #[must_use]
    pub const fn as_entry(&self) -> &ReactionEntry {
        &self.0
    }

    /// Consumes the wrapper and returns the reaction entry.
    #[must_use]
    pub fn into_entry(self) -> ReactionEntry {
        self.0
    }
}

impl fmt::Display for ProductEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
