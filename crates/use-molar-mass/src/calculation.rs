use use_chemical_formula::ChemicalFormula;

use crate::{
    AtomicMassLookup, ElementMassContribution, FormulaMolarMass, MassContributionSet,
    MolarMassValidationError,
};

/// A formula and atomic-mass lookup ready for molar-mass calculation.
#[derive(Clone, Debug, PartialEq)]
pub struct MolarMassCalculation {
    formula: ChemicalFormula,
    lookup: AtomicMassLookup,
}

impl MolarMassCalculation {
    /// Creates a calculation from a formula and explicit lookup table.
    #[must_use]
    pub const fn new(formula: ChemicalFormula, lookup: AtomicMassLookup) -> Self {
        Self { formula, lookup }
    }

    /// Creates a calculation using the RustUse standard atomic-mass table.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error if any formula element is missing
    /// from the RustUse atomic-mass table.
    pub fn with_standard_atomic_masses(
        formula: ChemicalFormula,
    ) -> Result<Self, MolarMassValidationError> {
        let lookup = AtomicMassLookup::from_formula(&formula)?;

        Ok(Self { formula, lookup })
    }

    /// Returns the source formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        &self.formula
    }

    /// Returns the atomic-mass lookup table.
    #[must_use]
    pub const fn lookup(&self) -> &AtomicMassLookup {
        &self.lookup
    }

    /// Calculates the formula molar mass.
    ///
    /// # Errors
    ///
    /// Returns a structured molar-mass validation error when a formula count is
    /// invalid, a count cannot be represented, an atomic mass is missing, or the
    /// calculated molar mass is invalid.
    pub fn calculate(&self) -> Result<FormulaMolarMass, MolarMassValidationError> {
        let mut contributions = Vec::new();

        for (symbol, count) in self.formula.element_counts() {
            if count == 0 {
                return Err(MolarMassValidationError::ZeroElementCount { symbol });
            }

            let count = u32::try_from(count).map_err(|_| {
                MolarMassValidationError::FormulaCountTooLarge {
                    symbol: symbol.clone(),
                    count,
                }
            })?;
            let atomic_mass = self.lookup.atomic_mass(&symbol).ok_or_else(|| {
                MolarMassValidationError::MissingAtomicMass {
                    symbol: symbol.clone(),
                }
            })?;

            contributions.push(ElementMassContribution::new(&symbol, atomic_mass, count)?);
        }

        FormulaMolarMass::new(
            self.formula.clone(),
            MassContributionSet::from_contributions(contributions),
        )
    }
}
