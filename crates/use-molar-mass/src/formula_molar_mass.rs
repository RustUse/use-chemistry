use std::fmt;

use use_chemical_formula::ChemicalFormula;

use crate::{MassContributionSet, MolarMass, MolarMassValidationError};

/// A formula with its calculated molar mass and element contributions.
#[derive(Clone, Debug, PartialEq)]
pub struct FormulaMolarMass {
    formula: ChemicalFormula,
    molar_mass: MolarMass,
    contributions: MassContributionSet,
}

impl FormulaMolarMass {
    /// Creates a formula molar mass from a formula and contribution set.
    ///
    /// # Errors
    ///
    /// Returns a molar-mass validation error if the contribution total is invalid.
    pub fn new(
        formula: ChemicalFormula,
        contributions: MassContributionSet,
    ) -> Result<Self, MolarMassValidationError> {
        let molar_mass = contributions.molar_mass()?;

        Ok(Self {
            formula,
            molar_mass,
            contributions,
        })
    }

    /// Returns the source formula.
    #[must_use]
    pub const fn formula(&self) -> &ChemicalFormula {
        &self.formula
    }

    /// Returns the calculated molar mass.
    #[must_use]
    pub const fn molar_mass(&self) -> MolarMass {
        self.molar_mass
    }

    /// Returns the element contribution set.
    #[must_use]
    pub const fn contributions(&self) -> &MassContributionSet {
        &self.contributions
    }
}

impl fmt::Display for FormulaMolarMass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.formula, self.molar_mass)
    }
}
