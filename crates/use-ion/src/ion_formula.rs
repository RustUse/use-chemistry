use std::fmt;

use use_chemical_formula::ChemicalFormula;

/// An ion-facing chemical formula wrapper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IonFormula(ChemicalFormula);

impl IonFormula {
    /// Creates an ion formula wrapper.
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

impl From<ChemicalFormula> for IonFormula {
    fn from(value: ChemicalFormula) -> Self {
        Self::new(value)
    }
}

impl AsRef<ChemicalFormula> for IonFormula {
    fn as_ref(&self) -> &ChemicalFormula {
        self.as_formula()
    }
}

impl fmt::Display for IonFormula {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
