use std::fmt;

use use_chemical_formula::ChemicalFormula;

/// A compound-facing chemical formula wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompoundFormula(ChemicalFormula);

impl CompoundFormula {
    /// Creates a compound formula wrapper.
    #[must_use]
    pub const fn new(formula: ChemicalFormula) -> Self {
        Self(formula)
    }

    /// Returns the wrapped formula.
    #[must_use]
    pub const fn as_formula(&self) -> &ChemicalFormula {
        &self.0
    }

    /// Consumes the wrapper and returns the formula.
    #[must_use]
    pub fn into_formula(self) -> ChemicalFormula {
        self.0
    }
}

impl From<ChemicalFormula> for CompoundFormula {
    fn from(value: ChemicalFormula) -> Self {
        Self::new(value)
    }
}

impl AsRef<ChemicalFormula> for CompoundFormula {
    fn as_ref(&self) -> &ChemicalFormula {
        self.as_formula()
    }
}

impl fmt::Display for CompoundFormula {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// An empirical formula wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmpiricalFormula(ChemicalFormula);

impl EmpiricalFormula {
    /// Creates an empirical formula wrapper.
    #[must_use]
    pub const fn new(formula: ChemicalFormula) -> Self {
        Self(formula)
    }

    /// Returns the wrapped formula.
    #[must_use]
    pub const fn as_formula(&self) -> &ChemicalFormula {
        &self.0
    }

    /// Consumes the wrapper and returns the formula.
    #[must_use]
    pub fn into_formula(self) -> ChemicalFormula {
        self.0
    }
}

impl From<ChemicalFormula> for EmpiricalFormula {
    fn from(value: ChemicalFormula) -> Self {
        Self::new(value)
    }
}

impl AsRef<ChemicalFormula> for EmpiricalFormula {
    fn as_ref(&self) -> &ChemicalFormula {
        self.as_formula()
    }
}

impl fmt::Display for EmpiricalFormula {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A molecular formula wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MolecularFormula(ChemicalFormula);

impl MolecularFormula {
    /// Creates a molecular formula wrapper.
    #[must_use]
    pub const fn new(formula: ChemicalFormula) -> Self {
        Self(formula)
    }

    /// Returns the wrapped formula.
    #[must_use]
    pub const fn as_formula(&self) -> &ChemicalFormula {
        &self.0
    }

    /// Consumes the wrapper and returns the formula.
    #[must_use]
    pub fn into_formula(self) -> ChemicalFormula {
        self.0
    }
}

impl From<ChemicalFormula> for MolecularFormula {
    fn from(value: ChemicalFormula) -> Self {
        Self::new(value)
    }
}

impl AsRef<ChemicalFormula> for MolecularFormula {
    fn as_ref(&self) -> &ChemicalFormula {
        self.as_formula()
    }
}

impl fmt::Display for MolecularFormula {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
